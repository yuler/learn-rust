// References and Borrowing

fn main() {
    // 1. Pass reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 2. Modify something borrowing
    let s = String::from("hello");
    change1(&s);

    // 3. Mutable References
    let mut s = String::from("hello");
    change2(&mut s);
    println!("{}", s);

    // 4. Only one mutable reference
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    {
        let r2 = &mut s; // Fix with scope
    }
    // println!("{}, {}", r1, r2);

    // 5. Can't borrow as mutable with borrowed as immutable
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // Error. Fix it: need r1 or r2 no longer used after
    // println!("{}, {}, and {}", r1, r2, r3);

    // 6. Dangling References
    // let reference_to_nothing = dangle();
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len() // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
}

fn change1(s: &String) {
    // s.push_str(", world") // Can't borrow mutable
}

fn change2(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope
