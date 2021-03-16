use std::io;

mod basic;
mod temp_converter;

fn main() {
    loop {
        let mut program_number = String::new();

        println!(
            "
Select the program
1. Basics
2. Temp Converter
3. Exit :( "
        );

        io::stdin()
            .read_line(&mut program_number)
            .expect("Input a number");

        let program_number: u32 = program_number
            .trim()
            .parse()
            .expect("An input number required");
        match program_number {
            1 => basic::main(),
            2 => temp_converter::main(),
            3 => break,
            _ => println!("invalid option"),
        };
    }
}
