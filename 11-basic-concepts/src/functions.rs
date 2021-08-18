// Functions

fn main() {
    another_function();

    param_function(5);

    add2_function(1);

    let x = five();
    println!("The value of x is {}", x);
    let x = plus_one(x);
    println!("The value of x is {}", x);
}

// Define Function
fn another_function() {
    println!("Another function.");
}

// Function Parameters
fn param_function(x: i32) {
    println!("The value of x is: {}", x)
}

// Function Bodies Contain Statements and Expressions
fn add2_function(num: i32) {
    let x = {
        let two = 2;
        two + num
    };

    println!("The result is: {}", x);
}

// Functions with Return Values
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
