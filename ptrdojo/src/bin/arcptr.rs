// Explanation:
// Arc<T> stands for "Atomic Reference Counted" and is similar to Rc<T>,
// but it is designed for multi-threaded scenarios.
// Thread-Safe: Arc uses atomic operations to manage the reference count, making it safe to use across threads.
// Challenges Solved:
// Thread Safety: Allows multiple threads to own and access the same data safely.
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let config = Arc::new(String::from("New string"));

    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let data_config = Arc::clone(&config);
        // data_config can be cloned here for every
        let handle = thread::spawn(move || {
            println!("Thread data: {:?}", data_clone);
            println!("Thread config: {:?}", data_config.split(" "));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
