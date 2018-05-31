// use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::sync::MutexGuard;
use std::thread;

fn main() {
    mutex_api();
    sharing_mutex_between_threads();
}

fn mutex_api() {
    let m = Mutex::new(5);

    {
        // To access the data inside the mutex, we use the lock method to acquire the lock
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        // MutexGuard is a smart pointer (i.e. implements Deref)
        *num = 6;
        // Once the guard goes out of scope, its lock is dropped
    }

    println!("m = {:?}", m);
}

fn sharing_mutex_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Without this line we get "capture of moved value: `counter`"
        // let counter = Rc::clone(&counter); // but doesn't work
        // ^^ the trait bound `std::rc::Rc<std::sync::Mutex<i32>>: std::marker::Send` is not satisfied
        let counter = Arc::clone(&counter); // but doesn't work
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