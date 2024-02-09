use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

struct Something {
    some_value: String,
}

impl Something {
    fn new(value: String) -> Self {
        Self { some_value: value }
    }

    fn say_hello(&self) {
        println!("hello");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Red
        }
    }
}

//  Clousers: anonymous functions

// const ADD_ONE_V2: u32 = |x: u32| -> u32 { x + 1 };
// const ADD_ONE_V3: u32 = |x| x + 1;
// const ADD_ONE_V4: u32 = |x| x + 1;

// It also takes arguments, in  3 diff ways, mutable borrow, immutable borrow, and ownership

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let add_one_v2 = |x: u32| x + 1;
    let number = add_one_v2(4);

    println!("number is : {}", number);

    let example_closure = |x| x;

    let _s = example_closure(String::from("hello")); // with first time compiler will infer the type
                                                     // let n = example_closure(5); Error

    // taking ownership of the variable in closure
    let list = vec![1, 2, 3, 4, 5, 6];

    println!("Before defining clouser: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let something = Something::new(String::from("Aman Pandey"));
    something.say_hello();
    println!("something struct and impl demo: {}", something.some_value);
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // closure that implements FnMut sort_by_key
    let mut rect_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    rect_list.sort_by_key(|r| r.width + r.height);
    println!("{:?}", rect_list);
}
