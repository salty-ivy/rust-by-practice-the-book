// Field struct

#[derive(Debug)] // this is to use Debug function which helps us to print the User instance or we can impl Debug method
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// associating a method to the Rectangle type via implementation

impl Rectangle {
    // each type uses multiple impl blocks
    fn area(&self) -> u32 {
        // &self is shorthand for self: &Self, which Self -> represents type written after impl
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // associated functions which do not take &self ( static methods or class methods ) and called like ::  eg: String::from("something")
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Tuple struct
struct Color(i32, i32, i32);

// unit like structures

struct AlwaysEqual;

fn main() {
    let user1 = User {
        // if let mut the values can be updated.
        active: true,
        username: String::from("soanso"),
        email: String::from("test@example.com"),
        sign_in_count: 2,
    };

    let user2 = User {
        username: String::from("new user"),
        email: String::from("new@example.com"),
        ..user1
    };

    let color = Color(0, 0, 0);
    let _black = color;

    let _subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 20,
        height: 20,
    };

    let sq = Rectangle::new_square(50);

    println!("{:?}", sq);

    println!("rectangle: {:?}", rect1);
    println!("area: {}, perimeter: {}", rect1.area(), rect1.perimeter());

    // User can also be return type of a function

    println!(
        "{}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
    println!(
        "{}, {}, {}, {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    println!("{:?}", user1); // using debug output format
    dbg!(user2); // dbg! macro takes the ownership and writes the output in stderr also prints file and line number
}
