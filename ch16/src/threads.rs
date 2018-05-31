use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // By joining (blocking) here, we first allow the spawned
    // thread to do its job, and only then continue
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // By joining (blocking) here, we allow the main thread
    // to do its thing, but ensure that the spawned thread
    // also completes before terminating
    // handle.join().unwrap();

    // NOTE: We can only join once, or else:
    // ^^^^^^ value used here after move

    // Without joining/blocking, the spawned thread may not
    // be ran at all or get terminated prematurely


    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
                            // ^^ may outlive borrowed value `v`
                            // (if move not specified)
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

}
