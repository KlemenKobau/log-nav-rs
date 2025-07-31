use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc::{self, Receiver, SendError},
    thread,
};

fn main() -> anyhow::Result<()> {
    let receiver = spawn_file_reader_thread()?;

    for e in receiver.iter() {
        println!("{}", e);
    }

    Ok(())
}

fn spawn_file_reader_thread() -> anyhow::Result<Receiver<String>> {
    let (sender, receiver) = mpsc::sync_channel::<String>(10);

    let file = File::open("log.in")?;
    let mut reader = BufReader::new(file);

    thread::spawn(move || {
        loop {
            let mut buffer = String::new();

            let res = reader.read_line(&mut buffer);
            if let Err(err) = res {
                eprintln!("Error reading line! {}", err);
            }

            // TODO clone could be removed by sending the buffer
            let send_res = sender.send(buffer);

            if let Err(SendError(error)) = send_res {
                eprintln!("Error while sending! {}", error);
            }
        }
    });

    Ok(receiver)
}
