use std::fs::read_to_string;

use fancy_regex::Regex;

fn main() {
    let mut java_logs = read_to_string("log.in").unwrap();
    let java_regex = read_to_string("java.txt").unwrap();

    let re = Regex::new(&java_regex).unwrap();

    loop {
        let caps = re.captures(&java_logs).unwrap().unwrap();
        let m = caps.get(0).unwrap();
        println!("{:?}", m.as_str());
        let end = m.end();

        java_logs = java_logs[end..].into();
    }
}
