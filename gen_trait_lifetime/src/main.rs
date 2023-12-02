// Generics is to avoid duplicaitons of code and to genralize the logic for incoming unknown types

// generic largest function code

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
