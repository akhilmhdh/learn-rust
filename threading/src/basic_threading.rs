use std::thread;
use std::time::Duration;

pub fn main() {
    let sample_thread = thread::spawn(move || {
        for i in 1..10 {
            println!("i goes from {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        println!("Hi this is outside thread {}", i);
        thread::sleep(Duration::from_millis(2));
    }

    // use join to wait for spawn thread to finish
    sample_thread.join().unwrap();
}
