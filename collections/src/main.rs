use std::collections::HashMap;

// collections vector, string, hashmap
#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let _v: Vec<i32> = Vec::new();  // annotations
    let mut v2 = vec![1, 2, 3]; //vec! macro to make it with inintal sets of values
    v2.push(5);
    v2.push(6);
    v2.push(7);

    let mut var = v2[3]; // cloning of the indexed value
    let mut_var = &mut v2[3];  // panics if out of index range is referenced
    var += 100;
    *mut_var += 1;  // derefrencing for mutability operations

    // println!("{}", v2[4]); // borrowing rules prevent this when having refecrences to vector 
    println!("{}", var);
    println!("{}", mut_var);
    println!("{}", v2[3]);

    let third: Option<&i32> = v2.get(4); // get method with index in to provides Option<&t> to use match on it

    match third {
        Some(t) => println!("The fourth element is {t}"),
        None => println!("There is no fourth element"),
    }


    // itereating over a vactor

    for i in &v2 {
        println!("{}", i);
    }

    let mut row: Vec<SpreadSheetCell> = Vec::new();
    row.push(SpreadSheetCell::Int(32));
    row.push(SpreadSheetCell::Float(3.22));
    row.push(SpreadSheetCell::Text(String::from("Address")));

    println!("row: {:?}, {:?}, {:?}", row[0], row[1], row[2]);


    // String Collections

    let data = "some string";

    let s = data.to_string(); // making String object
    let s2 = String::from(" hello world");

    let s3 = s + &s2;  // s lost its ownership
    let s4 = String::from("using format marco");

    let s5 = format!("{s3}--{s4}");  // doesn't take ownership uses reference and much better for complicating concatination

    for c in s5.chars(){
        println!("{}", c);
    }

    println!("from format macro: {s5}");

    println!("{}", s3);


    // Hash-Maps

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0); // copies to convert Option<&T> to Option<T>   unwrap_or(0) set the value or 0
    println!("{score}");

    // insert options take mover the values if variables are inserted
    let green_team = String::from("Green");
    let yellow_team = String::from("Yellow");

    scores.insert(green_team, 20);  // can't borrow
    scores.insert(yellow_team, 5);

    // println!("{green_team}, {yellow_team}"); ERROR

    // entry returns Entry

    scores.entry(String::from("Black")).or_insert(55);
    scores.entry(String::from("Blue")).or_insert(60);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
