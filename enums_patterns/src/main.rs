// enums give you a way of saying a value is one of a possible set of values.

#[derive(Debug)]
enum IpAddrKind { // this is custom data type now
    V4,  // variants
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    Vimg(String),
}


impl IpAddr {
    fn print_me(&self){
        println!("{:?}", &self);
    }
}


// dynaic multi-typing of enums  with Option<t> enum Option<t> enum with Some<t> and None variants comes preluded in the scope;

#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

// #[derive(Debug)]
// enum Message {
//     // Quit,  // similar to unit struct
//     Move {x: i32, y: i32},  // similar to regular struct
//     // Write(String), // tuple struct
//     // ChangeColor(i32, i32, i32, i32), // tuple struct
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn check_ip_address(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("{}:{}:{}:{}", a, b, c, d);
        },
        IpAddr::V6(some_string) => {
            println!("{}", some_string);
        },
        _ => { // other placeholder that matches all other possibilities.
            println!(" other match called ");
        },
    }
}

fn main() {

    let v4 = IpAddrKind::V4;
    let v6: IpAddrKind = IpAddrKind::V6;

    let some_numebr = Option::Some(5);
    let some_char = Option::Some("yoyo");
    let absent_number: Option<i32> = Option::None;  // explicit typing since None can't be infered

    println!("{:?}, {:?}, {:?}", some_char, some_numebr, absent_number);




    // let home_ip: IpAddr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let home_ip = IpAddr::V4(127, 0, 0, 1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    let imaginary: IpAddr = IpAddr::Vimg(String::from("1l2l2424"));

    home_ip.print_me();
    loopback.print_me();


    // consise workdlow with if let keyword  , equivalent to single arm match or instance checking
    if let IpAddr::V4(a, b, c, d) = home_ip {
        println!("if let block: {}, {}, {}, {}", a, b, c, d);
    }

    check_ip_address(&home_ip);
    check_ip_address(&loopback);
    check_ip_address(&imaginary);

    // let move_message: Message = Message::Move{ x: 3, y: -2 };

    // println!("{}, {}", move_message .x, move_message.y);

    println!("{:?}", home_ip);
    println!("{:?}", loopback);


    // println!("{:?}, {}", home_ip.kind, home_ip.address);
    println!("{:?}, {:?}", v4, v6);
}
