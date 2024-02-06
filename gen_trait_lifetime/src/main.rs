// Generics is to avoid duplicaitons of code and to genralize the logic for incoming unknown types

// generic largest function code


// Traits is like definig a common must have bevahior of methods, something like abstact class in python and interface in java

// trait for point

use std::fmt::Display;


trait DistanceFromOrigin{
    fn from_origin(&self) -> i32; // this methods needs to be implemented by any type that uses this trait on itself, also we can implement some default logic 
}

trait SumOfCoordinates {
    fn sum(&self) -> i32 {
        &self.x + &self.y
    }
}


struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {  // impl<T, U> tells impl its going to work on generic with T,U 
    fn x(&self)-> &T {
        &self.x
    }

    fn y(&self)-> &U {
        &self.y
    }
}

impl DistanceFromOrigin for Point<i32, i32> { // when other people using the crate they need to bring trait and the type together use something::{DistanceFromOrigin, Point};
    // fn hello(&self){
    //     println!("hello from point i32 type only");
    // }

    // must have method

    fn from_origin(&self) -> i32 {
        &self.x*&self.x + &self.y * &self.y
    }
}


impl SumOfCoordinates for Point<i32, i32>{}
// using a trait on Point


// traits as a parameter

fn use_traits(item: &impl SumOfCoordinates, item2: &(impl SumOfCoordinates + DistanceFromOrigin)){ // that means item is some time that has implemented the trait SumOfCoordinate
    item.sum();
}

// another version is trait Bound, also using + syntax to sepcift multiple triat bounds

fn use_trait_verbose<T: SumOfCoordinates + DistanceFromOrigin>(item: &T, item2: &T){  // this can help in having item of same type restriction as T generic implementor will make sure of it
    item.sum();
}

// redeifning trait bounds with where clause

fn use_trait_verbose_where<T, U>(t: &T, u: &U) -> Option<i32> where T: SumOfCoordinates, U: SumOfCoordinates + DistanceFromOrigin{
    println!("yooyoyoyyo");
    None
}

// lifetimes to use the corelation between parameters and return value
// life time annotation uses `'` and small lettered name afterwards like 'a
// here return type now will have the lifetime validation as much as its parameter had

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }
    else{
        y
    }
}


/*

lifttime elision rules.

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

*/

// rule one, 'a is assigned automatically to i param
// rule 2 is there's one param then lifetime of that param is assigned to output param 
fn longest_rule_1_and_2(s: &str) -> &str{
    &s[1..3]
}

// if rule 2 doesn't fit we specify explicitly
fn longest_rule_2<'a, 'b>(s1: &'a str, s2: &'b str) -> Vec<&'b str>{
    vec![]
}


// third rules for method onle if param is &self then output param lifetime will be that of &self.


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}



fn main() {
    let number_list: Vec<i32> = vec![34, 12, 34,24, 64, 3];
    let result  = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['s', 'w', 'a', 'd', 'g', 'b'];

    let result = largest(&char_list);

    // lifetime longest method concept potrayal

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);
}
