/*
 Concurrent programming, where different parts of a program execute independently,
 and parallel programming, where different parts of a program execute at the same time

 In most current operating systems, an executed program’s code is run in a process,
 and the operating system will manage multiple processes at once.
 Within a program, you can also have independent parts that run simultaneously.
 The features that run these independent parts are called threads.
*/

use std::thread;
use std::time::Duration;

// channels for message passing between threads
use std::sync::mpsc;
use std::sync::{Arc, Mutex}; // atomic reference counter SP for concurrent usecse of Rc<T>

fn main() {
    // all threads end if main thread is complete
    // thread scheduling is done by OS so no guarantee whether your thread will get the chance to run at all.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // calling join here would result same as single threaded

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates
    handle.join().unwrap(); // waiting for this thread to complete before spawned finished before main finishes

    let v = vec![1, 2, 3];

    // since we can't tell how and when the spawn thread runs ( before or after ) and when it ends
    // we can't be sure to reference V in closure so we move it in the environment
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel(); // transmitter and reciever tuple return from channel

    let tx1 = tx.clone(); // new producer

    // thread::spawn(move || {
    //     let value = String::from("message from spawned thread");

    //     // send also takes ownership of the values
    //     tx.send(value).unwrap(); // sending to main thread
    // });

    // let received = rx.recv().unwrap();
    // println!("got: {}", received)

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from(" spawn"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // treating as an iterator
    // iterator will wait for messages while they come and closes as tx closes
    for received in rx {
        println!("got: {}", received);
    }

    // shared state concurrency

    /*
    Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
    To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
    The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
    */

    let m = Mutex::new(5);

    // lock returns MutexGaurd as smar pointer which impl deref to point the data at * operations
    // and drop when it goes out scope it unlocks automatically

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0)); // SP atomic version of Rc<T> so counter can be moved to multiple threads
                                           // to ensure multiple ownership at once since it will be bounced around the threads
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
