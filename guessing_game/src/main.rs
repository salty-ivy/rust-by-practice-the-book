use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // ownership

    let s = String::from("hello world");
    let mut s1 = s;
    s1.push_str(" python world");
      // s.clone makes a deep copy but this mover the ownership to s1 and s is no longer valid.
    println!("{}", s1);

    let mutable_s1 = &mut s1; // at a time only once mutable reference for a value is allowed or multiple immutable references.

    // let mutable_s1_part_2: &mut String = &mut s1;  // this is gonna throw error

    let lenght2: usize = calculate_length_by_reference(mutable_s1); // &mut is mutable reference since reference is also immutable by deafult

    let mutable_s1_part_2: &mut String = &mut s1; // now this is allowed as mutable_s1 is out of scope as it has been used for the last time above this line as it can be detected

    println!("{}", mutable_s1_part_2);


    println!("{}", lenght2);
    println!("{}", s1);
    let length: usize = calculate_length(s1);
    println!("{}", length);
    // println!("{}", s1); owership moved to the function hece not usable anymore.

    {
        let s2 = String::from("scope test");
        println!("{}", s2);
    }
    // println!("{}", s2);  // s2 went out of scope

    // String slices ( these doesn't take onwership if tossed around with & and their type is &str )


    let to_be_sliced: String = String::from("to be sliced");

    let word = first_word(&to_be_sliced);

    // let word2: &str = first_word(&s1[1..3]);  // [1..5] exclusive at the end [1..=5] inclusice at the end

    // println!("{}", word2);
    
    // to_be_sliced.clear(); // error  since first_word returns a sliced reference to actual owner to_be_sliced and word used the ref to original string can't be cleared.

    
    println!("{}, {}", to_be_sliced, word);


    let tuple: (i32, f64, u8) = (5, 6.4, 2); // destructring is also possible
    let (x, y, z) = tuple; // () means empty tuple or unit.
    let individual = tuple.2;
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // every element must be of same type [type, size];
    println!("{individual}");
    println!(" {x} {y} {z} ");
    println!("{}", a[4]);
    println!("{}", tuple.2);

    let x = 200;

    if x == 20 {
        println!("if ran!");
    }else if x >20 {
        print!("else if ran!");
    }else{
        println!("else ran !");
    }

    let y = if x>=100 {x} else {101};
    println!("{y}");

    println!("Enter number index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("not");
    let index: usize = index.trim().parse().expect("Not a valid index"); // usize use

    let element = a[index];
    println!("{element}");

    let secrete_number = rand::thread_rng().gen_range(1..=100);
    // println!("secrete number: {secrete_number}");
    loop{
        println!("guess a number, and input here: ");
        let mut guess = String::new(); // by defailt we variables are immutable but here we use mut to make them otherwise.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){ // parse returns Result type with 2 variants., shadowing the guess var by let, also helps in type conversion
            Ok(num) => num, // arm 1
            Err(_) => continue, // _ means match all the Error instead of any specefic one, arm 2.
        };
        println!("Your guess: {guess}");

        match guess.cmp(&secrete_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it hurray!!!!");
                break;
            }
        }
    }
}

fn calculate_length(some_string: String) -> usize {
    let length = some_string.len();
    length // if return &length means passing a dangling reference which is not allowed
}

fn calculate_length_by_reference(some_string: &mut String) -> usize { // reference passing is borrowing 
    some_string.push_str("changed via reference");
    some_string.len()
}

// string slicing 

fn first_word(some_string: &str) -> &str{
    let string_bytes = some_string.as_bytes();

    for (i, &item) in string_bytes.iter().enumerate(){
        if item == b' '{
            return &some_string[0..i];
        }
    }

    &some_string[..]
}