use anyhow::{Context, Result};
use std::{collections::HashMap, fmt::Display, str::FromStr};
use thiserror::Error;


#[derive(PartialEq, Debug)]
pub enum UciMessage {
    UciOk,
    ReadyOk,
    Info {
        cp: Option<isize>,
        mate: Option<isize>,
        depth: Option<isize>,
        seldepth: Option<isize>,
        nodes: Option<isize>,
        time: Option<isize>,
        multipv: Option<isize>,
        pv: Option<Vec<String>>,
    },
    Perft {
        m: String,
        nodes: u64,
    },
    Option { name: String, opt_type: OptionType },
    FinishedThinkingSignal,
}

#[derive(PartialEq, Debug, Clone)]
pub enum OptionType {
    Check {
        default: bool,
    },
    Spin {
        default: isize,
        min: isize,
        max: isize,
    },
    Combo {
        default: String,
        options: Vec<String>,
    },
    Button,
    String {
        default: String,
    },
}


impl OptionType {
    fn new(opt_type: String, line: String) -> Result<Self> {
        Ok(match opt_type.as_str() {
            "check" => OptionType::new_check(line)?,
            "spin" => OptionType::new_spin(line)?,
            "combo" => OptionType::new_combo(line)?,
            "button" => OptionType::new_button()?,
            "string" => OptionType::new_string(line)?,
            _ => return Err(UciParseError::ParseError.into()),
        })
    }

    fn new_check(line: String) -> Result<Self> {
        let words = vec!["default"];
        let values = parse_line_values(line, words)?;
        Ok(OptionType::Check {
            default: values["default"].unwrap(),
        })
    }

    fn new_spin(line: String) -> Result<Self> {
        let words = vec!["default", "min", "max"];
        let values = parse_line_values(line, words)?;
        Ok(OptionType::Spin {
            default: values["default"].unwrap(),
            min: values["min"].unwrap(),
            max: values["max"].unwrap(),
        })
    }

    fn new_combo(line: String) -> Result<Self> {
        let words = vec!["default"];
        let values = parse_line_values(line.clone(), words)?;
        let line: Vec<&str> = line.split_whitespace().collect();
        let mut options = Vec::new();
        // TODO: Check if combo options can have spaces, in which case this will give incorrect results
        for ix in 0..line.len() {
            if line[ix] == "var" {
                options.push(line[ix + 1].to_string());
            }
        }
        Ok(OptionType::Combo {
            default: values["default"].clone().unwrap(),
            options,
        })
    }

    fn new_button() -> Result<Self> {
        Ok(OptionType::Button)
    }

    fn new_string(line: String) -> Result<Self> {
        let words = vec!["default"];
        let values = parse_line_values(line, words)?;
        Ok(OptionType::String {
            default: values["default"].clone().unwrap(),
        })
    }
}


#[derive(Error, Debug)]
pub enum UciParseError {
    ParseError,
}

impl Display for UciParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            UciParseError::ParseError => "error parsing uci command",
        };
        f.write_str(data)
    }
}


pub fn parse_uci(line: String) -> Result<UciMessage> {
    let line = line.trim().to_string();
    let command = line.split_whitespace().next().unwrap_or("");
    match command {
        "info" => parse_info_line(line),
        "uciok" => Ok(UciMessage::UciOk),
        "readyok" => Ok(UciMessage::ReadyOk),
        "option" => parse_option_line(line),
        "Nodes" => Ok(UciMessage::FinishedThinkingSignal),
        "bestmove" => Ok(UciMessage::FinishedThinkingSignal),
        _ => {
            match command.chars().next() {
                Some('a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') => parse_perft_line(line),
                _ => Err(UciParseError::ParseError.into())
            }
        }
    }
}

fn parse_line_values<T: FromStr + Default>(
    line: String,
    words: Vec<&str>,
) -> Result<HashMap<String, Option<T>>> {
    let line: Vec<&str> = line.split_whitespace().collect();
    let mut values = HashMap::with_capacity(words.len());
    for word in words.iter() {
        let mut i = line.iter();
        let value = match i.position(|x: &&str| x == word) {
            Some(ix) => match line.get(ix + 1) {
                Some(v) => v.parse::<T>().ok(),
                None => Some(T::default()),
            },
            None => None,
        };
        values.insert(word.to_string(), value);
    }
    Ok(values)
}

fn parse_info_line(line: String) -> Result<UciMessage> {
    let words = vec![
        "cp", "depth", "nodes", "seldepth", "mate", "time", "multipv",
    ];
    let values = parse_line_values(line.clone(), words)?;
    Ok(UciMessage::Info {
        cp: values["cp"],
        mate: values["mate"],
        depth: values["depth"],
        nodes: values["nodes"],
        time: values["time"],
        multipv: values["multipv"],
        seldepth: values["seldepth"],
        pv: parse_pv(line),
    })
}

fn parse_perft_line(line: String) -> Result<UciMessage> {
    let mut parts = line.split(':');
    let m = parts.next().context("failed to parse perft")?;
    let nodes = parts.next().context("failed to parse perft")?.trim().parse().context("failed to parse perft")?;
    Ok(UciMessage::Perft {
        m: m.to_string(),
        nodes,
    })
}

fn parse_pv(line: String) -> Option<Vec<String>> {
    let line: Vec<&str> = line.split_whitespace().collect();
    let mut pv = Vec::new();
    let mut i = line.iter();
    match i.position(|x: &&str| *x == "pv") {
        Some(_) => {}
        None => return None,
    };
    for word in i {
        pv.push(word.to_string());
    }
    Some(pv)
}

fn parse_option_line(line: String) -> Result<UciMessage> {
    // TODO: handle `name`s with spaces (i.e. `option name Clear Hash type button`)
    let words = vec!["name", "type"];
    let values = parse_line_values(line.clone(), words)?;
    Ok(UciMessage::Option {
        name: values["name"].clone().unwrap(),
        opt_type: OptionType::new(values["type"].clone().unwrap(), line)?,
    })
}
