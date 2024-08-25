use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use std::sync::mpsc;

fn main(){

    let th1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from th1...{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main function execution
    for xdx in 5..10{
        println!("Lets say pro frm main...{}", xdx);
    }

    th1.join().unwrap();

    println!("Lets access same data from diff threads...");

    let tcnt = Arc::new(Mutex::new(0));

    let mut handles = vec![];


    for _i in 0..10 {
        let tcnt = Arc::clone(&tcnt);
        let hdl = thread::spawn(move || {
            let mut num = tcnt.lock().unwrap();
            *num += 1;
       });
        handles.push(hdl)
    }

    for hdl in handles{
        hdl.join().unwrap();
    }

    println!("result: {}", *tcnt.lock().unwrap());

    // mpsc::channel: Creates a channel with a sender (tx) and a receiver (rx).
    // tx.send: Sends data from one thread to another via the channel.
    // rx.recv: Receives data from the channel, blocking until a message is available.


    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    handle.join().unwrap();


}
