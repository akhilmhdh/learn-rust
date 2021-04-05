use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    /*
    arc is atomic referece count
    Meaning at the cost of performace they provide thread safety
    Combine with mutex and get a thread share variable
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    /*
    Clone and unlock to access variables
    */
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
