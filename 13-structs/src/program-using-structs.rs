// Program Using Structs

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // With Tuples
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect)
    );

    // With Structs
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect)
    );

    // Print Struct Instance
    println!("{:?}", rect);
    println!("{:#?}", rect);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
