use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn thread_based() {
    let handle = thread::spawn(|| {
        for idx in 1..5 {
            println!("Basic thread: {}", idx);
            thread::sleep(Duration::from_secs(2))
        }
    });
    handle.join().unwrap();
    println!("Starting the multi threads...");

    let mut handles = vec![];
    for idx in 1..3 {
        let handle = thread::spawn(move || {
            println!("Thread {idx:?} working");
            thread::sleep(Duration::from_secs(2));
            for jdx in 1..5 {
                thread::sleep(Duration::from_secs(2));
                println!("I am inside {}", idx)
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let (te, re) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Message1"),
            String::from("Message2"),
            String::from("Message3"),
        ];
        for val in vals {
            te.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for recieved in re {
        println!("Recieved: {recieved:?}")
    }
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        // Counter
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{}", num);
        });
        handles.push(handle);
    }
    println!("Counter is: {:?}", counter);
}
