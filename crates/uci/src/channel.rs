use std::{io, sync::mpsc, thread};

pub fn spawn_stdin() -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if tx.send(buf).is_err() {
            break;
        };
    });

    rx
}
