// Date Types

use std::io;

fn main() {
    // Convert string to number, must add a type annotation
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Scalar Types
    // Four primary scalar types: integers, floating-point numbers, Booleans, and characters
    // 1. Integer Literals in Rust
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // (u8 only)

    // 2. Floating-Point Types: f32 f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 3. Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 4. Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    // 1. Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 2. Array Type
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let arr = [3; 5]; // => let arr = [3, 3, 3, 3, 3];

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];

    let a = [1, 2, 3, 4, 5];

    // Invalid Array Element Access
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
