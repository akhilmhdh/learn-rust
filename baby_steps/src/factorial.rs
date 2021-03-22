use std::io;

pub fn main() {
    loop {
        let mut input = String::new();
        println!("Type a number to find factorial / Type exit to exit");

        io::stdin().read_line(&mut input).expect("Input a number");

        if input.trim() == "exit" {
            break;
        }

        let input: usize = input.trim().parse().expect("Number is expected");

        let mut factorial_arr: Vec<i64> = vec![1; input + 1];
        factorial_arr[0] = 1;

        for i in 1..factorial_arr.len() {
            factorial_arr[i] = factorial_arr[i - 1] * (i as i64);
        }

        println!("Factorial is: {}", factorial_arr[input]);
    }
}
