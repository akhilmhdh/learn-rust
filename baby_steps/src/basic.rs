/**
 * Just basic stuff
 * variable mutability
 * fn signature
 * type casting
 */
pub fn main() {
    let x = 5;
    let x = x + 10;

    println!("Hello x:{}", x);

    let mut y = 10;
    println!("Hello y:{}", y);
    y = 20;
    println!("Hello y:{}", y);

    let tup = (0, 1, 2);
    println!("Tuple {}", tup.0);

    let arr = [3; 5];
    println!("Array {}", arr[0]);

    println!("Dummy function: {}", some_dummy_function());

    if x == 5 {
        println!("This x is 5 dumbo");
    }
}

fn some_dummy_function() -> i32 {
    let x = 5;
    x
}
