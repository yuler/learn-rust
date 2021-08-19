// Structs

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Creating an instance of the User struct
    let mut user = User {
        email: String::from("foo@boo.com"),
        username: String::from("baz"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
    println!("{}", user.active);

    // Changing the value in the email field of a User instance
    user.email = String::from("baz@boo.com");
    println!("{}", user.email);

    // Function returns a User instance
    fn create_user(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    // Creating a new User instance using some of the values from user
    let user1 = User {
        username: String::from("yuler"),
        ..user
    };

    // Tuple Structs without Named Fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
