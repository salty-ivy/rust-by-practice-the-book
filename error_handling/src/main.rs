
// unrecoverable errors with panic! macro
// recoverable errors with Result<T,E>
// run RUST_BACKTRACE=1 cargo run
use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io::{self, Read};

// Error propogation

fn _read_username_from_file()-> Result<String, io::Error>{
    let file_result = File::open("username.txt");
    let mut target_file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match target_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? operator does what above function does, returns T Result vairant of Ok(T) is ok else return Err(e) from the whole function
// ? also works on Option, returns T of Some(T) or return None from the whole function

// ? Err(e) goes through From function that will convert your Error type to type defiend by return type of the function, match can't do this.

fn _shorthand_username_from_file()-> Result<String, io::Error> {
    // let mut target_file = File::open("username.txt")?;
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;  // if Ok will return T of Ok(T) else Err(E) whole we wont catch T and return Ok(usename)
    Ok(username)
}

fn _short_read_from_file()-> Result<String, io::Error>{
    fs::read_to_string("username.tct")  // std library fs directly provide a function that implements above manaually created functions
}


fn main() {

    let initial_file_result = File::open("file.txt");

    let _initial_file = match initial_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        },
    };

    // helper method unwrap() of result type which returns T of  Ok(T) else run panic itself

    let _initial_file = File::open("file2.txt").unwrap();

    // expect() one the result helper method, to customize panic method

    let _initial_file: File = File::open("file2.txt").expect("Custom panic message: file not found");
}
