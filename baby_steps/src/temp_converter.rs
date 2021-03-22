use std::io;
/**
 * Temperature converter program
 * Celsius <-> Fahrenheit converter
 */
pub fn main() {
    let mut converter_type = String::new();
    let mut input_temp = String::new();

    loop {
        /*
            io::read_line appends to the variable
            as its loop this causes panic on parsing as digit
            to avoid this we must clear the variable each time
        */
        converter_type.clear();
        input_temp.clear();
        println!(
            "
Select converter
1. Celsius to Fahrenheit
2. Fahrenheit to Celsius 
3. Exit"
        );

        io::stdin()
            .read_line(&mut converter_type)
            .expect("Expected input");

        let converter_type: i32 = converter_type.trim().parse().expect("Enter a number");

        if converter_type == 3 {
            break;
        }
        println!("Input temperature in degree");
        io::stdin()
            .read_line(&mut input_temp)
            .expect("Expected temperature");

        let input_temp: i32 = input_temp.trim().parse().expect("Enter a number");

        let output_temp = match converter_type {
            1 => (input_temp * 9 / 5) + 32,
            2 => (input_temp - 32) * 5 / 9,
            _ => 0,
        };

        println!("Output Temperature is {}", output_temp);
    }
}
