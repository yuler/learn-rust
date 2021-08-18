// Ownership

fn main() {
    // 1. s is store in heap
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 2. s1 has been invalidated when move
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // 3. deep copy, clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 4. Copy trait, stack copy
    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);

        // 5. Ownership and functions
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function and so is no longer valid here
                            // println!("{}", s); moved
        let x = 5; // x comes into scope
        makes_copy(x); // copy
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    // 6. Return Values and Scope
    // Returning values can also transfer ownership.

    // 7. Return multiple values using a tuple

    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
