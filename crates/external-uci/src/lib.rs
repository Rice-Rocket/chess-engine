mod parse;

use crate::parse::{parse_uci, OptionType, UciMessage};

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use std::{fmt::Display, process::Stdio, sync::{mpsc, Arc, Mutex}};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines}, process::{Child, ChildStdin, ChildStdout, Command}};


#[async_trait]
pub trait ExternalUciCapable {
    async fn start_uci(&mut self) -> Result<()>;

    async fn new_game(&mut self) -> Result<()>;

    async fn set_position(&mut self, position: &str) -> Result<()>;
    
    async fn set_position_moves(&mut self, position: &str, moves: Vec<String>) -> Result<()>;

    async fn go_infinite(&mut self) -> Result<()>;

    async fn go_depth(&mut self, plies: usize) -> Result<()>;

    async fn go_time(&mut self, ms: usize) -> Result<()>;

    async fn go_mate(&mut self, mate_in: usize) -> Result<()>;

    async fn go_perft(&mut self, depth: usize) -> Result<()>;

    async fn stop(&mut self) -> Result<()>;

    async fn quit(&mut self) -> Result<()>;

    async fn kill(&mut self) -> Result<()>;

    async fn get_evaluation_block(&mut self) -> Option<UciEvaluation>;

    async fn get_perfts_block(&mut self) -> Result<Vec<UciPerftResults>>;

    async fn get_evaluation(&mut self) -> Option<UciEvaluation>;

    async fn get_options(&mut self) -> Result<Vec<UciEngineOption>>;

    async fn get_perfts(&mut self) -> Result<Vec<UciPerftResults>>;

    fn get_bestmove(&mut self) -> Option<String>;

    async fn set_option(&mut self, option: String, value: String) -> Result<()>;
}


pub struct ExternalUci {
    process: Child,
    stdin: ChildStdin,
    state: UciState,
}

impl ExternalUci {
    pub async fn new(exe_path: &str) -> Result<Self> {
        let (process, stdin, stdout) = spawn_process(exe_path, vec![])?;
        let state = UciState::new(stdout).await;
        Ok(ExternalUci {
            process,
            state,
            stdin,
        })
    }

    pub async fn new_with_args(exe_path: &str, args: Vec<String>) -> Result<Self> {
        let (process, stdin, stdout) = spawn_process(exe_path, args)?;
        let state = UciState::new(stdout).await;
        Ok(ExternalUci {
            process,
            state,
            stdin,
        })
    }

    async fn send_command(&mut self, command: String) -> Result<()> {
        self.stdin.write_all(command.as_bytes()).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    async fn _expect_state(&mut self, exp_state: &UciStateEnum) -> Result<()> {
        let state = self.state.state.lock().expect("couldn't acquire state lock");
        if *exp_state == *state {
            return Ok(());
        }
        bail!("engine didn't respond with {:?}", exp_state)
    }

    async fn expect_state(&mut self, exp_state: UciStateEnum) -> Result<()> {
        for _ in 0..10 {
            match self._expect_state(&exp_state).await {
                Ok(_) => return Ok(()),
                Err(_) => tokio::time::sleep(std::time::Duration::from_millis(100)).await,
            };
        }
        bail!("engine didn't respond with {:?}", exp_state)
    }

    async fn expect_uciok(&mut self) -> Result<()> {
        self.expect_state(UciStateEnum::Initialized).await
    }

    async fn expect_readyok(&mut self) -> Result<()> {
        self.expect_state(UciStateEnum::Ready).await
    }

    async fn set_state(&mut self, new_state: UciStateEnum) -> Result<()> {
        // TODO: Return old state
        let mut state = self.state.state.lock().expect("couldn't acquire lock");
        *state = new_state;
        Ok(())
    }
}


fn spawn_process(exe_path: &str, args: Vec<String>) -> Result<(Child, ChildStdin, ChildStdout)> {
    let mut cmd = Command::new(exe_path);
    cmd.args(args);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    let mut proc = cmd.spawn()?;
    let stdout = proc.stdout.take().expect("no stdout available");
    let stdin = proc.stdin.take().expect("no stdin available");
    Ok((proc, stdin, stdout))
}


#[async_trait]
impl ExternalUciCapable for ExternalUci {
    async fn start_uci(&mut self) -> Result<()> {
        self.send_command("uci\n".to_string()).await?;
        self.expect_uciok().await?;
        self.send_command("isready\n".to_string()).await?;
        self.expect_readyok().await?;
        Ok(())
    }

    async fn new_game(&mut self) -> Result<()> {
        self.send_command("ucinewgame\n".to_string()).await?;
        self.set_state(UciStateEnum::Initialized).await?;
        self.send_command("isready\n".to_string()).await?;
        self.expect_readyok().await?;
        Ok(())
    }

    async fn set_position(&mut self, fen: &str) -> Result<()> {
        let cmd = format!("position fen {}\n", fen);
        self.send_command(cmd.to_string()).await
    }

    async fn set_position_moves(&mut self, fen: &str, moves: Vec<String>) -> Result<()> {
        let mut cmd = format!("position fen {} moves", fen);
        for m in moves {
            cmd.push_str(&format!(" {}", m));
        }
        cmd.push('\n');
        self.send_command(cmd.to_string()).await
    }

    async fn go_infinite(&mut self) -> Result<()> {
        self.send_command("go infinite\n".to_string()).await?;
        self.set_state(UciStateEnum::Thinking).await?;
        Ok(())
    }

    async fn go_depth(&mut self, depth: usize) -> Result<()> {
        self.send_command(format!("go depth {}\n", depth).to_string())
            .await?;
        self.set_state(UciStateEnum::Thinking).await?;
        Ok(())
    }

    async fn go_time(&mut self, ms: usize) -> Result<()> {
        self.send_command(format!("go movetime {}\n", ms).to_string())
            .await?;
        self.set_state(UciStateEnum::Thinking).await?;
        Ok(())
    }

    async fn go_mate(&mut self, mate_in: usize) -> Result<()> {
        self.send_command(format!("go mate {}\n", mate_in).to_string())
            .await?;
        self.set_state(UciStateEnum::Thinking).await?;
        Ok(())
    }

    async fn go_perft(&mut self, depth: usize) -> Result<()> {
        self.send_command(format!("go perft {}\n", depth).to_string()).await?;
        self.set_state(UciStateEnum::Thinking).await?;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        self.send_command("stop\n".to_string()).await?;
        self.set_state(UciStateEnum::Initialized).await?;
        Ok(())
    }

    async fn quit(&mut self) -> Result<()> {
        self.send_command("quit\n".to_string()).await?;
        self.set_state(UciStateEnum::Initialized).await?;
        Ok(())
    }

    async fn kill(&mut self) -> Result<()> {
        self.process.kill().await.unwrap();
        Ok(())
    }

    async fn get_evaluation_block(&mut self) -> Option<UciEvaluation> {
        if let Ok(UciEvent::Finished) = self.state.receiver.recv() {
            let eval = self.state.evaluation.lock().expect("couldn't acquire lock");
            (*eval).as_ref().cloned()
        } else {
            None
        }
    }

    async fn get_perfts_block(&mut self) -> Result<Vec<UciPerftResults>> {
        if let Ok(UciEvent::Finished) = self.state.receiver.recv() {
            let perfts = self.state.perfts.lock().expect("couldn't acquire lock");
            Ok(perfts.clone())
        } else {
            Err(anyhow!("failed to receive from uci"))
        }
    }

    async fn get_evaluation(&mut self) -> Option<UciEvaluation> {
        let ev = self.state.evaluation.lock().expect("couldn't acquire lock");
        (*ev).as_ref().cloned()
    }

    async fn get_options(&mut self) -> Result<Vec<UciEngineOption>> {
        let options = self.state.options.lock().expect("couldn't acquire lock");
        Ok(options.clone())
    }

    async fn get_perfts(&mut self) -> Result<Vec<UciPerftResults>> {
        let perfts = self.state.perfts.lock().expect("couldn't acquire lock");
        Ok(perfts.clone())
    }

    fn get_bestmove(&mut self) -> Option<String> {
        match self.state.receiver.try_recv() {
            Ok(UciEvent::Finished) => {
                let bestmove = self.state.bestmove.lock().expect("couldn't acquire lock");
                Some(bestmove.clone())
            },
            Err(mpsc::TryRecvError::Empty) => {
                None
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                panic!("failed to receive from uci, sender disconnected");
            }
        }
    }

    async fn set_option(&mut self, option: String, value: String) -> Result<()> {
        let cmd = format!("setoption name {} value {}\n", option, value);
        self.send_command(cmd).await
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UciEvaluation {
    pub score: isize,
    pub mate: isize,
    pub depth: isize,
    pub nodes: isize,
    pub seldepth: isize,
    pub multipv: isize,
    pub pv: Vec<String>,
    pub time: isize,
}

impl Default for UciEvaluation {
    /// Create evaluation with empty values
    fn default() -> Self {
        UciEvaluation {
            score: 0,
            mate: 0,
            depth: 0,
            nodes: 0,
            seldepth: 0,
            multipv: 0,
            pv: vec![],
            time: 0,
        }
    }
}

impl Display for UciEvaluation {
    /// The alternate ("{:#}") operator will add the moves in pv to the output
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "score: {} mate: {} depth: {} nodes: {} seldepth: {} multipv: {} time: {}",
            self.score, self.mate, self.depth, self.nodes, self.seldepth, self.multipv, self.time
        ))?;
        if f.alternate() {
            f.write_fmt(format_args!("\npv: {}", self.pv.join(", ")))?;
        }
        Ok(())
    }
}


#[derive(PartialEq, Debug)]
enum UciStateEnum {
    Uninitialized,
    Initialized,
    Ready,
    Thinking,
}

#[derive(PartialEq, Debug)]
enum UciEvent {
    Finished,
}


#[derive(PartialEq, Debug, Clone)]
pub struct UciEngineOption {
    pub name: String,
    pub opt_type: OptionType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct UciPerftResults {
    pub m: String,
    pub nodes: u64,
}

struct UciState {
    state: Arc<Mutex<UciStateEnum>>,
    evaluation: Arc<Mutex<Option<UciEvaluation>>>,
    options: Arc<Mutex<Vec<UciEngineOption>>>,
    perfts: Arc<Mutex<Vec<UciPerftResults>>>,
    bestmove: Arc<Mutex<String>>,
    receiver: mpsc::Receiver<UciEvent>,
}

impl UciState {
    async fn new(stdout: ChildStdout) -> Self {
        let ev = Arc::new(Mutex::new(None));
        let state = Arc::new(Mutex::new(UciStateEnum::Uninitialized));
        let options = Arc::new(Mutex::new(Vec::new()));
        let perfts = Arc::new(Mutex::new(Vec::new()));
        let bestmove = Arc::new(Mutex::new(String::new()));
        let stdout = BufReader::new(stdout).lines();
        let (tx, rx) = mpsc::channel();
        let engstate = UciState {
            state: state.clone(),
            evaluation: ev.clone(),
            options: options.clone(),
            perfts: perfts.clone(),
            bestmove: bestmove.clone(),
            receiver: rx,
        };

        tokio::spawn(async move {
            Self::process_stdout(stdout, state.clone(), ev.clone(), options.clone(), perfts.clone(), bestmove.clone(), tx).await
        });

        engstate
    }

    async fn process_stdout(
        mut stdout: Lines<BufReader<ChildStdout>>,
        state: Arc<Mutex<UciStateEnum>>,
        ev: Arc<Mutex<Option<UciEvaluation>>>,
        options: Arc<Mutex<Vec<UciEngineOption>>>,
        perfts: Arc<Mutex<Vec<UciPerftResults>>>,
        bestmove: Arc<Mutex<String>>,
        sender: mpsc::Sender<UciEvent>,
    ) {
        while let Some(line) = stdout.next_line().await.unwrap() {
            match parse_uci(line) {
                Ok(UciMessage::UciOk) => {
                    let mut state = state.lock().expect("couldn't acquire state lock");
                    *state = UciStateEnum::Initialized;
                },
                Ok(UciMessage::ReadyOk) => {
                    let mut state = state.lock().expect("couldn't acquire state lock");
                    *state = UciStateEnum::Ready;
                },
                Ok(UciMessage::Info {
                    cp,
                    mate,
                    depth,
                    nodes,
                    seldepth,
                    time,
                    multipv,
                    pv,
                }) => {
                    let mut ev = ev.lock().expect("couldn't acquire ev lock");
                    let def_ev = UciEvaluation::default();
                    let prev_ev = match ev.as_ref() {
                        Some(ev) => ev,
                        None => &def_ev,
                    };
                    *ev = Some(UciEvaluation {
                        score: cp.unwrap_or(prev_ev.score),
                        mate: mate.unwrap_or(prev_ev.mate),
                        depth: depth.unwrap_or(prev_ev.depth),
                        nodes: nodes.unwrap_or(prev_ev.nodes),
                        seldepth: seldepth.unwrap_or(prev_ev.seldepth),
                        multipv: multipv.unwrap_or(prev_ev.multipv),
                        pv: pv.unwrap_or(prev_ev.pv.clone()),
                        time: time.unwrap_or(prev_ev.time),
                    });
                },
                Ok(UciMessage::Option { name, opt_type }) => {
                    let mut options = options.lock().expect("couldn't acquire options lock");
                    options.push(UciEngineOption { name, opt_type });
                },
                Ok(UciMessage::Perft { m, nodes }) => {
                    let mut perfts = perfts.lock().expect("couldn't acquire perfts lock");
                    perfts.push(UciPerftResults { m, nodes });
                },
                Ok(UciMessage::FinishedThinkingSignal) => {
                    let mut state = state.lock().expect("couldn't acquire state lock");
                    *state = UciStateEnum::Ready;
                    if sender.send(UciEvent::Finished).is_err() {
                        return;
                    }
                },
                Ok(UciMessage::BestMove { m }) => {
                    let mut bestmove = bestmove.lock().expect("couldn't acquire bestmove lock");
                    *bestmove = m;
                    let mut state = state.lock().expect("couldn't acquire state lock");
                    *state = UciStateEnum::Ready;
                    if sender.send(UciEvent::Finished).is_err() {
                        return;
                    }
                },
                _ => continue,
            }
        }
    }
}
