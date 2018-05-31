use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // transmitter | receiver
    let (tx, rx) = mpsc::channel();

    // Obtain a new transmitter for the same receiver
    let tx1 = mpsc::Sender::clone(&tx);

    // move the transmitter to a different thread
    thread::spawn(move || {
        // Dispatch 'hi' to the receiver
        let val = String::from("hi");
        tx1.send(val).unwrap();
        // sending moves `vals`, so the following fails compilation:
        // println!("vals is {}", vals);

        let vals = vec![
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // At this point, the thread stops, `tx` goes out of
        // scope, and therefore, the receiver knows it will
        // not receive any more messages
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv() is blocking / try_recv non-blocks and returns Result instead
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Receive any remaining values from the main thread
    for received in rx {
        println!("Got: {}", received);
        // At some point, the transmitter goes out of scope
        // and this loop automatically terminates! \o/
    }
}