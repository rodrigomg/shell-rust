use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        print!(">_ ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut iterator = words.into_iter();
        let mut command;
        match iterator.next() {
            Some(arg) => command = Command::new(arg),
            None => command = Command::new("clear")
        }
        for e in iterator {
            command.arg(e);
        }
        match command.spawn() {
            Ok(_) => println!("cool"),
            Err(e) => println!("{:?}", e),

        }
        //let status = command.status().expect("Fallo");
        //match status.code() {
        //    Some(code) => println!("Exited with status code: {}", code),
        //    None       => println!("Process terminated by signal")
        //}
    }
}
