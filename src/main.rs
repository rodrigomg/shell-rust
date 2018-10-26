use std::io;
use std::process::Command;
fn main() {
    loop {
        println!(">_ ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut iterator = words.into_iter();
        let mut list_dir;
        match iterator.next() {
            Some(command) => list_dir = Command::new(command),
            None => list_dir = Command::new("clear")
        }
        for e in iterator {
            list_dir.arg(e);
        }
        list_dir.status().expect("Fallo");
    }
}
