// Generics is to avoid duplicaitons of code and to genralize the logic for incoming unknown types

// generic largest function code


// Traits is like definig a common must have bevahior of methods, something like abstact class in python and interface in java

// trait for point


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
}
