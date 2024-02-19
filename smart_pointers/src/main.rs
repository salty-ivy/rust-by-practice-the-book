/*
In this suggestion, “indirection” means that instead of storing a value directly,
we should change the data structure to store the value indirectly by storing a pointer to the value instead.

Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
This means we can put a Box<T> inside the Cons variant instead of another List value directly.
The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
Conceptually, we still have a list, created with lists holding other lists,
but this implementation is now more like placing the items next to one another rather than inside one another.

The Box<T> type is a smart pointer because it implements the Deref trait,
which allows Box<T> values to be treated like references.

https://doc.rust-lang.org/book/ch15-02-deref.html


Reference counter Rc<T>

We use the Rc<T> type when we want to allocate some data on the heap for multiple
parts of our program to read and we can’t determine at compile time which part will finish using the data last.
If we knew which part would finish last, we could just make that part the data’s owner,
and the normal ownership rules enforced at compile time would take effect.

*/

use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping the value: {}", self.data);
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    // use of Rc<T> instead of Box<T> as it allows mutiple owners ( single threaded ) also uses heap and best for the below case.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));  a moved to me
    // let c = Cons(4, Box::new(a)); but used here again thus this gives compile time error

    let ar = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let br = Cons(3, Rc::clone(&ar));
    let cr = Cons(4, Rc::clone(&ar));

    /*
    We could have called a.clone() rather than Rc::clone(&a),
    but Rust’s convention is to use Rc::clone in this case.
    The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.
    By using Rc::clone for reference counting,
    */

    let cp = CustomSmartPointer {
        data: String::from(" custom pointer demo data"),
    };

    /*
    and we can’t call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.
    */

    drop(cp);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:?}", list);

    // Box<T> like a normal reference
    let x = 4;
    // let y = &x;
    let y = Box::new(x);
    assert_eq!(4, x);
    assert_eq!(4, *y); // derefrencing
                       /*
                       The main difference between Listing 15-7 and Listing 15-6
                       is that here we set y to be an instance of a Box<T> pointing to a copied value of x
                       rather than a reference pointing to the value of x
                       */
}
