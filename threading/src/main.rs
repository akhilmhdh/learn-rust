use std::io;

mod basic_threading;
mod channel;

fn main() {
    loop {
        let mut input = String::new();

        println!(
            "
Threading
1. Basic Threading
2. Channel
0. Exit
        "
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Program number must be given");

        let input: i32 = input.trim().parse().expect("To be a number");
        match input {
            0 => break,
            1 => basic_threading::main(),
            2 => channel::main(),
            _ => println!("invalid option"),
        }
    }
}
