use std::io;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

trait Executable {
    fn new(string_command: &str) -> Cmd;
    fn build_the_command(&self) -> Command;
}

impl<'a> Executable for Cmd<'a> {
    fn new(string_command: &str) -> Cmd {
        let mut words: Vec<&str> = string_command.split_whitespace().collect();
        let modifiers: Vec<&str> = words.drain(1..).collect();
        Cmd {
            binary: words[1],
            args: modifiers,
        }
    }
    fn build_the_command(&self) -> Command {
        let mut command_executor = Command::new(self.binary);
        for e in &self.args {
            command_executor.arg(e);
        };
        command_executor
    }
}

fn get_input() -> String {
    print!(">_ ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn split_commands(input: &str) -> Vec<&str> {
    input.split(';').collect()
}

fn exec_the_command(string_command: &str) {
    //let mut command_executor = build_the_command(&string_command);
    //match command_executor.spawn() {
    //    Ok(_) => print!(""),
    //    Err(e) => println!("{:?}", e),
    //}
}

fn exec_commands(commands: Vec<&str>) {
    for string_command in commands {
        let cmd = Cmd::new(string_command);
        //exec_the_command(&string_command)
        println!("{:?}", cmd);
    }
}

fn main() {
    loop{
        let input = get_input();
        let commands: Vec<&str> = split_commands(&input);
        exec_commands(commands);
    }
}
