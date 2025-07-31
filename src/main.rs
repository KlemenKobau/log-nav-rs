use anyhow::Result;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::mpsc::{self, Receiver, SendError},
    thread,
};

fn main() -> Result<()> {
    let receiver = spawn_file_reader_thread("log.in")?;

    for e in receiver.iter() {
        print!("{}", e);
    }

    Ok(())
}

fn spawn_file_reader_thread<P: Into<PathBuf>>(path: P) -> Result<Receiver<String>> {
    let path = path.into();

    let (line_sender, line_receiver) = mpsc::sync_channel::<String>(10);
    let notify_receiver = spawn_notify_thread(&path);

    thread::spawn(move || {
        let file = File::open(&path).expect("Error opening file!");
        let mut reader = BufReader::new(file);

        loop {
            loop {
                // TODO fix this. This will not read lines without \n, so it will not read the last line in the file.
                // also problematic if the line is 1Gb big...
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Err(SendError(line)) = line_sender.send(buffer) {
                            eprintln!("Receiver dropped: {}", line);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        break;
                    }
                }
            }

            if let Err(err) = notify_receiver.recv() {
                eprintln!("Error waiting for notification! {}", err);
            }
        }
    });

    Ok(line_receiver)
}

fn spawn_notify_thread<P: Into<PathBuf>>(
    path: P,
) -> Receiver<Result<notify::Event, notify::Error>> {
    let (notify_sender, notify_receiver) = mpsc::channel();

    let watch_path = path.into();
    thread::spawn(move || {
        let mut watcher =
            notify::recommended_watcher(notify_sender).expect("Error creating watcher!");

        watcher
            .watch(&watch_path, RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        loop {
            thread::park();
        }
    });

    notify_receiver
}
