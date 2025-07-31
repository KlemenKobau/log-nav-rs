use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
    path::PathBuf,
    sync::mpsc::{self, Receiver, SendError},
    thread,
};

fn main() -> anyhow::Result<()> {
    let receiver = spawn_file_reader_thread("log.in")?;

    for e in receiver.iter() {
        print!("{}", e);
    }

    Ok(())
}

fn spawn_file_reader_thread<P: Into<PathBuf>>(path: P) -> anyhow::Result<Receiver<String>> {
    let path = path.into();

    let (line_sender, line_receiver) = mpsc::sync_channel::<String>(10);

    thread::spawn(move || {
        let (notify_sender, notify_receiver) = mpsc::channel();

        let file = File::open(&path).expect("Error opening file!");
        let mut reader = BufReader::new(file);

        let mut watcher =
            notify::recommended_watcher(notify_sender).expect("Error creating watcher!");

        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        let notifier = notify_receiver
            .iter()
            .filter(|x| match x {
                Ok(Event {
                    kind: notify::EventKind::Modify(_),
                    ..
                }) => true,
                Ok(_) => false,
                Err(_) => false,
            })
            .map(|_| ());

        for _ in once(()).chain(notifier) {
            loop {
                // TODO fix this. This will not read lines without \n, so it will not read the last line in the file.
                // also problematic if the line is 1Gb big...
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        break;
                    }
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
        }
    });

    Ok(line_receiver)
}
