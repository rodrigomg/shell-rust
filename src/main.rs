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
        //list_dir.status().expect("process failed to execute");
        //println!("{:?}", words);
        //println!("{:?}", iterator.next());
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
    /* let output = Command::new("sh")
       .arg("-c")
       .arg("echo hello")
       .output()
       .expect("failed to execute process");
       let hello = output.stdout;
       println!("{:?}",hello);*/
}
