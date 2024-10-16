use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn basic() {
    // we are spawning
    let handle = thread::spawn(|| {
        for idx in 1..5 {
            println!("Basic thread: {}", idx);
            thread::sleep(Duration::from_millis(500));
            // creates a simple time delay
        }
    });
    handle.join().unwrap();
    // this thread is not linking with any
    // other thread
}

pub fn basic_with_for() {
    for idx in 1..5 {
        println!("Basic thread: {}", idx);
        thread::sleep(Duration::from_millis(500));
    }
}

pub fn thread_with_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Thread with move: {:?}", v);
    });
    // println!("Cannot print here: {v:?}");
    handle.join().unwrap();
}

pub fn thread_sleep() {
    let handle = thread::spawn(|| {
        println!("Thread sleep: Start");
        thread::sleep(Duration::from_secs(2));
        println!("Thread sleep: End");
    });
    handle.join().unwrap();
}

pub fn multi_threads() {
    let mut handles = vec![];
    // threads are similarly created here
    for idx in 1..3 {
        let handle = thread::spawn(move || {
            println!("Thread {idx:?} working");
            thread::sleep(Duration::from_secs(2));
            for jdx in 1..5 {
                thread::sleep(Duration::from_secs(3));
                println!("I am inside idx: {idx:?} and sleep on jdx: {jdx:?}");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        // each handle object is joined
        handle.join().unwrap();
    }
    // what will happen if the threads are not joined
}

pub fn thread_channels() {
    let (rx, tx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the thread...");
        thread::sleep(Duration::from_secs(5));
        rx.send(val).unwrap();
    });

    let recieved = tx.recv().unwrap();
    println!("Recieved: {}", recieved);
}

pub fn multi_value_channel() {
    let (tx, rx) = mpsc::channel();
    // if the move is not used, then tx and rx
    // may live for lesser time than thread below
    thread::spawn(move || {
        let vals = vec![
            String::from("Message1"),
            String::from("Message2"),
            String::from("Message3"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for recieved in rx {
        println!("Recieved: {recieved:?}");
    }
}

pub fn arc_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // Sharing and updating state between multiple threads using `Arc` and `Mutex`.
    for _ in 0..5 {
        // the above loop is in main thread
        let counter = Arc::clone(&counter);
        // starts ownership sharing using clone
        let handle = thread::spawn(move || {
            // counter is moved into each thread
            let mut num = counter.lock().unwrap();
            // Lock the Mutex before modifying the counter
            *num += 1;
            // value of the counter loc is updated
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
        // By joining the threads, you ensure that the main thread waits for all spawned threads to finish their work:
    }

    println!("Final count: {}", *counter.lock().unwrap());
}

pub fn deadlock_example() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);

    let handle1 = thread::spawn(move || {
        let lk1 = m1.lock().unwrap(); // acquires lock on mutex1
        thread::sleep(Duration::from_secs(1));
        let lk3 = m2.lock().unwrap();
        // below printing is unlikelly
        println!("Thread acquires both lock: {}, {}", lk1, *lk3);
    });
    // Clone Arc references to be used in the second thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    // Spawn the second thread
    let handle2 = thread::spawn(move || {
        // Thread 2 locks mutex2 first
        let lock2 = m2.lock().unwrap(); // Acquires lock on mutex2
                                        // Simulates some processing time (delays acquisition of the next lock)
        thread::sleep(Duration::from_millis(100));
        // Thread 2 then tries to lock mutex1
        let _lock1 = m1.lock().unwrap(); // Tries to acquire lock on mutex1
                                         // If it succeeds, it prints this message (unlikely to happen in this deadlock scenario)
        println!("Thread 2 acquired both locks: {}, {}", lock2, *_lock1);
    });

    handle2.join().unwrap();
    handle1.join().unwrap();
}

pub fn parallel_map_example() {
    let data = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];
    let result = Arc::new(Mutex::new(vec![]));
    // here the empty vector is first created as mutex

    for num in data {
        let result = Arc::clone(&result);
        // mutex is cloned
        let handle = thread::spawn(move || {
            let squared = num * num;
            // push is done through lock
            result.lock().unwrap().push(squared);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Squared Results: {:?}", *result.lock().unwrap());
}

/// Example 9: Thread Pool (Manually)
pub fn thread_pool_example() {
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread Pool - Thread {}", i);
            thread::sleep(Duration::from_millis(1000));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_test() {
        basic()
    }
}
