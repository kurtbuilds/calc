use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut acc = 0f64;
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,    // with ^Z
            Ok(s) => acc += s.parse::<f64>().unwrap(),
        }
    }
    if acc.round() == acc {
        println!("{}", acc.round() as i64);
    } else {
        println!("{}", acc);
    }
}