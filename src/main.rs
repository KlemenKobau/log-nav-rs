use std::io::{IsTerminal, Read, stdin};

fn main() {
    let mut buffer = [0; 1000];
    let mut stdin = stdin();

    loop {
        let read = stdin.read(&mut buffer).unwrap();
        if read != 0 {
            println!("{:?}", buffer);
        }
    }
}
