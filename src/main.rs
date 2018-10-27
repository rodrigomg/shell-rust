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
        let commands: Vec<&str> = input.split(';').collect();
        for command in commands {
            let words: Vec<&str> = command.split_whitespace().collect();
            let mut iterator = words.into_iter();

            let mut command_executor;
            match iterator.next() {
                Some(arg) => command_executor = Command::new(arg),
                None => command_executor = Command::new("clear")
            }
            for e in iterator {
                command_executor.arg(e);
            }
            match command_executor.spawn() {
                Ok(_) => println!("cool"),
                Err(e) => println!("{:?}", e),

            }
        }
    }
}
