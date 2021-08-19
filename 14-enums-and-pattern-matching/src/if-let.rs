// Concise Control Flow with if let

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let
    if let Some(3) = some_u8_value {
        println!("three")
    }
}
