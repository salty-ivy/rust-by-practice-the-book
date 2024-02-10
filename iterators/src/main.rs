//  iterators does squencing over an iterator
// if lang doesn't support one we make a look like i=0; i<n .....
// if iterator looping over then is easier
/*
 The iter method produces an iterator over immutable references.
 If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
 Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.
*/

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
