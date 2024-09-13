    // Clone Arc references to be used in the first thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    
    // Spawn the first thread
    let handle1 = thread::spawn(move || {
        // Thread 1 locks mutex1 first
        let lock1 = m1.lock().unwrap();  // Acquires lock on mutex1
        // Simulates some processing time (delays acquisition of the next lock)
        thread::sleep(Duration::from_millis(100));
        // Thread 1 then tries to lock mutex2
        let _lock2 = m2.lock().unwrap();  // Tries to acquire lock on mutex2
        // If it succeeds, it prints this message (unlikely to happen in this deadlock scenario)
        println!("Thread 1 acquired both locks: {}, {}", lock1, *_lock2);
    });
    
    // Clone Arc references to be used in the second thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    
    // Spawn the second thread
    let handle2 = thread::spawn(move || {
        // Thread 2 locks mutex2 first
        let lock2 = m2.lock().unwrap();  // Acquires lock on mutex2
        // Simulates some processing time (delays acquisition of the next lock)
        thread::sleep(Duration::from_millis(100));
        // Thread 2 then tries to lock mutex1
        let _lock1 = m1.lock().unwrap();  // Tries to acquire lock on mutex1
        // If it succeeds, it prints this message (unlikely to happen in this deadlock scenario)
        println!("Thread 2 acquired both locks: {}, {}", lock2, *_lock1);
    });
    
    // Wait for both threads to complete (which they won't in this deadlock scenario)
    handle1.join().unwrap();
    handle2.join().unwrap();

}

### 

Here's a structured tutorial covering the basics to advanced use cases of threads in Rust. We'll go through 10 examples, all implemented in `lib.rs`, with the results demonstrated in `main.rs`. Rust's threading is based on the concept of lightweight tasks, where threads can run concurrently. We'll cover common use cases like basic threading, thread communication, and synchronization.

### Project Structure

```
my_threading_project/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

### 1. **Cargo.toml**

```toml
[package]
name = "my_threading_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### 2. **lib.rs**

```rust
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

/// Example 1: Basic Threading
pub fn basic_threading() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Basic Thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handle.join().unwrap();
}

/// Example 2: Thread with Move Keyword
pub fn thread_with_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Thread with Move: {:?}", v);
    });

    handle.join().unwrap();
}

/// Example 3: Thread Sleep and Delay
pub fn thread_sleep() {
    let handle = thread::spawn(|| {
        println!("Thread Sleep: Start");
        thread::sleep(Duration::from_secs(2));
        println!("Thread Sleep: End");
    });

    handle.join().unwrap();
}

/// Example 4: Multiple Threads
pub fn multiple_threads() {
    let mut handles = vec![];

    for i in 1..4 {
        let handle = thread::spawn(move || {
            println!("Thread {}: Working", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Example 5: Channels for Communication
pub fn thread_with_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

/// Example 6: Multiple Values via Channel
pub fn multiple_values_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Message 1"),
            String::from("Message 2"),
            String::from("Message 3"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}

/// Example 7: Arc and Mutex (Shared State)
pub fn arc_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
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

    println!("Final Counter: {}", *counter.lock().unwrap());
}

/// Example 8: Deadlock Example (Be cautious)
pub fn deadlock_example() {
    // Create two Mutex objects wrapped in Arc (Atomic Reference Counting) to allow multiple ownership.
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // Clone Arc references to be used in the first thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);

    // Spawn the first thread
    let handle1 = thread::spawn(move || {
        // Thread 1 locks mutex1 first
        let lock1 = m1.lock().unwrap();  // Acquires lock on mutex1
        // Simulates some processing time (delays acquisition of the next lock)
        thread::sleep(Duration::from_millis(100));
        // Thread 1 then tries to lock mutex2
        let _lock2 = m2.lock().unwrap();  // Tries to acquire lock on mutex2
        // If it succeeds, it prints this message (unlikely to happen in this deadlock scenario)
        println!("Thread 1 acquired both locks: {}, {}", lock1, *_lock2);
    });

    // Clone Arc references to be used in the second thread
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);

    // Spawn the second thread
    let handle2 = thread::spawn(move || {
        // Thread 2 locks mutex2 first
        let lock2 = m2.lock().unwrap();  // Acquires lock on mutex2
        // Simulates some processing time (delays acquisition of the next lock)
        thread::sleep(Duration::from_millis(100));
        // Thread 2 then tries to lock mutex1
        let _lock1 = m1.lock().unwrap();  // Tries to acquire lock on mutex1
        // If it succeeds, it prints this message (unlikely to happen in this deadlock scenario)
        println!("Thread 2 acquired both locks: {}, {}", lock2, *_lock1);
    });

    // Wait for both threads to complete (which they won't in this deadlock scenario)
    handle1.join().unwrap();
    handle2.join().unwrap();
}
// Mutex Usage: Mutex is used to ensure that only one thread can access the critical section at a time.
// Arc Usage: Arc (Atomic Reference Counted) is used to share ownership of the Mutex between threads safely.
// Potential for Deadlock: The problem arises when handle1 locks mutex1 and waits for mutex2, while handle2 locks mutex2 and waits for mutex1. Both threads are now stuck waiting for each other to release their respective locks.
// Sleep to Simulate Race Condition: thread::sleep is used to make the deadlock situation more likely to occur by giving each thread time to acquire its first lock before attempting to acquire the second lock.

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

/// Example 10: Parallel Map with Threads
pub fn parallel_map_example() {
    let data = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];
    let result = Arc::new(Mutex::new(vec![]));

    for num in data {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let squared = num * num;
            result.lock().unwrap().push(squared);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Squared Results: {:?}", *result.lock().unwrap());
}
```

In Rust, the `move` keyword and `mpsc::channel` are significant concepts when working with threading and concurrency. Here's what each one does and why they're important:

### 1. `move` Keyword in Threading

The `move` keyword in Rust is used when spawning a new thread (or creating closures in general) to indicate that the closure should take ownership of the variables it captures from its surrounding environment. This is particularly important in multi-threaded programming because it ensures that data is safely transferred to the new thread without risking data races or dangling references.

#### Significance of `move` in Threading

- **Ownership Transfer**: When spawning a thread with `thread::spawn`, the `move` keyword ensures that all captured variables from the outer scope are moved (i.e., transferred) into the thread's scope. This prevents potential data races because ownership is moved, and the original variables in the parent thread are no longer accessible.
- **Thread Safety**: Rust's ownership model and the `move` keyword ensure that data is either completely owned by a single thread or properly synchronized across multiple threads using concurrency primitives (e.g., `Arc<Mutex<T>>`). This guarantees thread safety at compile time.

#### Example of `move` in Threads

```rust
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // The `move` keyword allows us to take ownership of `numbers` inside the thread.
        for number in numbers {
            println!("{}", number);
        }
    });

    // The following line would cause a compile-time error because `numbers` has been moved.
    // println!("{:?}", numbers);

    handle.join().unwrap();
}
```

In this example:

- The `move` keyword ensures that `numbers` is moved into the thread. Without `move`, Rust would not allow access to `numbers` inside the thread closure because it could lead to data races if `numbers` were used in both the main thread and the spawned thread.

### 2. `mpsc::channel` for Message Passing

`mpsc::channel` stands for **multiple producer, single consumer** channel. It is a concurrency primitive provided by Rust to allow safe and efficient message passing between threads.

#### Significance of `mpsc::channel`

- **Thread Communication**: Channels are a way for threads to communicate with each other by sending messages. This avoids sharing state directly and reduces the complexity of thread synchronization.
- **Decoupling**: The channel mechanism allows threads to be decoupled from each other. One thread can produce data and another can consume it without needing to share memory directly or synchronize access.
- **Synchronization**: Channels handle synchronization for you, meaning you don't need to worry about locks or other synchronization mechanisms. The `send` and `recv` operations block when necessary to ensure safe communication.

#### Example of `mpsc::channel`

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and move the sender (tx) into it
    thread::spawn(move || {
        let vals = vec!["hello", "from", "the", "thread"];

        for val in vals {
            tx.send(val).unwrap(); // Send each value over the channel
            thread::sleep(Duration::from_millis(200)); // Simulate some work
        }
    });

    // Receive values from the channel
    for received in rx {
        println!("Received: {}", received);
    }
}
```

In this example:

- `mpsc::channel()` creates a channel with a **sender** (`tx`) and a **receiver** (`rx`).
- We move `tx` (the sender) into a spawned thread using `move`.
- The spawned thread sends multiple messages through the channel using `tx.send()`.
- The main thread waits and receives messages using `rx.recv()`, which blocks until a message is received.

### Key Takeaways

1. **`move` Keyword**:
   
   - Ensures safe data transfer between threads by moving ownership of variables into the new thread.
   - Prevents data races and dangling references.

2. **`mpsc::channel`**:
   
   - Provides a safe way for threads to communicate via message passing.
   - Decouples threads by eliminating the need for direct state sharing and complex synchronization.
   - Handles synchronization internally, simplifying concurrent programming.

Together, these features provide a robust foundation for building safe, concurrent, and efficient programs in Rust.

Threading is a powerful tool for solving problems that involve concurrent or parallel execution, allowing for more efficient use of resources and faster execution times. Here are some interesting real-world problems that can be effectively solved using threading, along with examples:

### 1. **Web Server Handling Multiple Clients**

**Problem**: A web server needs to handle multiple client requests simultaneously to provide a responsive experience. If each request were handled sequentially, the server would be slow and unresponsive.

**Solution**: Use threading to handle each client request in a separate thread. This allows the server to serve multiple clients concurrently without blocking.

**Example**:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    stream.write(&buffer).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Spawn a new thread for each client connection
        thread::spawn(move || {
            handle_client(stream);
        });
    }
}
```

**Explanation**: Here, a new thread is spawned for each incoming client connection. This allows the server to handle multiple clients concurrently.

### 2. **Parallel Processing of Large Data Sets**

**Problem**: Processing large data sets (e.g., applying a complex transformation to each element in a large array) can be slow if done sequentially.

**Solution**: Use threading to divide the data set into smaller chunks and process each chunk in parallel.

**Example**:

```rust
use std::thread;

// Function to perform some computation
fn compute(numbers: &[u32]) -> u32 {
    numbers.iter().sum()
}

fn main() {
    let numbers: Vec<u32> = (1..=100_000).collect();
    let chunk_size = 10_000;
    let mut handles = vec![];

    for chunk in numbers.chunks(chunk_size) {
        // Clone the chunk to move into the thread
        let chunk = chunk.to_vec();

        // Spawn a new thread for each chunk
        let handle = thread::spawn(move || {
            let result = compute(&chunk);
            println!("Sum of chunk: {}", result);
            result
        });

        handles.push(handle);
    }

    // Collect the results from each thread
    let total_sum: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Total sum: {}", total_sum);
}
```

**Explanation**: This example divides a large list of numbers into chunks and processes each chunk in a separate thread. This allows for faster computation by utilizing multiple CPU cores.

### 3. **Real-Time Data Streaming (e.g., Stock Market)**

**Problem**: In financial applications, it is important to process real-time stock market data streams simultaneously for different stocks or financial instruments.

**Solution**: Use threading to handle multiple data streams concurrently.

**Example**:

```rust
use std::thread;
use std::time::Duration;

// Simulate data stream processing
fn process_data_stream(stock: &str) {
    for i in 0..10 {
        println!("Processing data for {}: Tick {}", stock, i);
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let stocks = vec!["AAPL", "GOOG", "AMZN"];

    let mut handles = vec![];

    for stock in stocks {
        // Clone the stock symbol to move into the thread
        let stock = stock.to_string();

        let handle = thread::spawn(move || {
            process_data_stream(&stock);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

**Explanation**: This example spawns a new thread for each stock symbol to simulate real-time data processing for multiple stocks concurrently.

### 4. **Concurrent Web Scraping**

**Problem**: Web scraping involves downloading and parsing web pages, which can be time-consuming if done sequentially.

**Solution**: Use threading to scrape multiple web pages concurrently, speeding up the scraping process.

**Example**:

```rust
use std::thread;
use reqwest::blocking::get;
use std::time::Duration;

fn scrape_url(url: &str) -> Result<(), reqwest::Error> {
    let response = get(url)?;
    println!("Scraped {}: {}", url, response.status());
    Ok(())
}

fn main() {
    let urls = vec![
        "https://www.example.com",
        "https://www.rust-lang.org",
        "https://www.github.com",
    ];

    let mut handles = vec![];

    for url in urls {
        let url = url.to_string();
        let handle = thread::spawn(move || {
            match scrape_url(&url) {
                Ok(_) => println!("Successfully scraped {}", url),
                Err(err) => eprintln!("Error scraping {}: {}", url, err),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

**Explanation**: In this example, each URL is scraped in a separate thread. This allows for concurrent HTTP requests, reducing the total time needed to scrape multiple web pages.

### 5. **Background Task Processing (e.g., Sending Emails or Notifications)**

**Problem**: In web applications, certain tasks like sending emails or notifications should be done asynchronously in the background to avoid blocking the main thread.

**Solution**: Use a thread to handle background tasks like sending emails or notifications.

**Example**:

```rust
use std::thread;
use std::time::Duration;

fn send_email(email: &str, message: &str) {
    println!("Sending email to {}: {}", email, message);
    thread::sleep(Duration::from_secs(2)); // Simulate email sending delay
    println!("Email sent to {}", email);
}

fn main() {
    let emails = vec!["user1@example.com", "user2@example.com"];

    let mut handles = vec![];

    for email in emails {
        let email = email.to_string();
        let handle = thread::spawn(move || {
            send_email(&email, "Hello! This is a background notification.");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All emails have been sent.");
}
```

**Explanation**: Each email is sent in a separate thread to avoid blocking the main thread. This is useful for web servers or applications that need to handle user requests without delay.

### 6. **Simulating a Multi-Player Game Server**

**Problem**: In a multi-player online game, each player might perform actions independently and concurrently.

**Solution**: Use threads to handle actions for each player concurrently, ensuring the game server remains responsive.

**Example**:

```rust
use std::thread;
use std::time::Duration;

// Simulate game actions for a player
fn simulate_player(player_id: u32) {
    for i in 0..5 {
        println!("Player {} is performing action {}", player_id, i);
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let mut handles = vec![];

    for player_id in 1..=4 {
        let handle = thread::spawn(move || {
            simulate_player(player_id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All players have completed their actions.");
}
```

**Explanation**: Each player's actions are simulated in a separate thread, allowing for concurrent gameplay simulation.

### Key Takeaways

- **Threading** is useful for problems involving concurrent or parallel processing.

- Rust's ownership model and safety guarantees make it easier to write safe, concurrent programs.

- Real-world applications of threading include web servers, data processing, real-time applications, web scraping, and background task processing.

- Proper use of threading can significantly improve the efficiency, responsiveness, and performance of applications.
  
  Testing multi-threaded applications can be challenging due to the non-deterministic nature of concurrent execution. However, there are strategies and techniques for writing reliable tests for threaded code. Below are several examples that illustrate different ways to test threaded code in Rust:

### 1. **Basic Thread Execution Testing**

  A simple test to ensure that threads are created and run correctly.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_execution() {
        let handle = thread::spawn(|| {
            for i in 1..5 {
                println!("Thread is running: {}", i);
                thread::sleep(Duration::from_millis(100));
            }
        });

        assert!(handle.join().is_ok()); // Ensure the thread executed without panic
    }
}
```

  **Explanation**: This test spawns a thread that prints messages. The test checks if the thread completes without any errors using `join()`, which waits for the thread to finish.

### 2. **Testing Thread Synchronization with `Mutex`**

  Testing for thread synchronization issues (e.g., race conditions) using `Mutex` to safely share data between threads.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_thread_synchronization() {
        let counter = Arc::new(Mutex::new(0));
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

        assert_eq!(*counter.lock().unwrap(), 10); // Check if counter reached 10
    }
}
```

  **Explanation**: This test creates multiple threads that increment a shared counter. The `Arc` and `Mutex` are used to safely share and mutate the counter across threads. The test verifies that all threads increment the counter correctly.

### 3. **Testing with `mpsc::channel` for Inter-Thread Communication**

  Testing inter-thread communication using Rust's `mpsc` (multiple producer, single consumer) channel.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_communication() {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            for i in 1..5 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        let mut received_values = vec![];
        for received in rx {
            received_values.push(received);
        }

        handle.join().unwrap();
        assert_eq!(received_values, vec![1, 2, 3, 4]); // Verify that all values were received
    }
}
```

  **Explanation**: This test creates a thread that sends values to the main thread via a channel. The main thread collects and verifies the received values.

### 4. **Testing Deadlock Prevention**

  Testing to ensure that code is free from deadlocks. Deadlocks occur when two or more threads are waiting indefinitely for each other to release resources.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_no_deadlock() {
        let resource1 = Arc::new(Mutex::new(0));
        let resource2 = Arc::new(Mutex::new(0));

        let r1 = Arc::clone(&resource1);
        let r2 = Arc::clone(&resource2);

        let handle1 = thread::spawn(move || {
            let _lock1 = r1.lock().unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
            let _lock2 = r2.lock().unwrap();
        });

        let r1 = Arc::clone(&resource1);
        let r2 = Arc::clone(&resource2);

        let handle2 = thread::spawn(move || {
            let _lock2 = r2.lock().unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
            let _lock1 = r1.lock().unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        assert!(true); // If we reach here, no deadlock occurred
    }
}
```

  **Explanation**: This test attempts to create a potential deadlock situation but uses timed delays to avoid it. The test passes if both threads complete without deadlocking.

### 5. **Testing Thread Pool Execution**

  Testing a thread pool implementation to ensure tasks are executed correctly and concurrently.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    struct ThreadPool {
        workers: Vec<thread::JoinHandle<()>>,
        sender: mpsc::Sender<Box<dyn FnOnce() + Send>>,
    }

    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);

            for _ in 0..size {
                let receiver = Arc::clone(&receiver);
                workers.push(thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    job();
                }));
            }

            ThreadPool { workers, sender }
        }

        fn execute<F>(&self, job: F)
        where
            F: FnOnce() + Send + 'static,
        {
            self.sender.send(Box::new(job)).unwrap();
        }
    }

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..8 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                thread::sleep(Duration::from_millis(50));
            });
        }

        thread::sleep(Duration::from_secs(1)); // Allow time for all tasks to complete
        assert_eq!(*counter.lock().unwrap(), 8); // Ensure all tasks were executed
    }
}
```

  **Explanation**: This test checks that a custom thread pool correctly schedules and executes tasks concurrently.

### 6. **Testing Thread Safety with `RwLock`**

  Testing read-write locks to ensure safe access to shared data in a multi-threaded environment.

  **Example:**

```rust
#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use std::thread;

    #[test]
    fn test_rwlock_safety() {
        let data = Arc::new(RwLock::new(5));
        let mut handles = vec![];

        // Multiple readers
        for _ in 0..5 {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let read_data = data.read().unwrap();
                assert_eq!(*read_data, 5);
            });
            handles.push(handle);
        }

        // One writer
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_data = data.write().unwrap();
            *write_data += 1;
        });
        handles.push(handle);

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*data.read().unwrap(), 6); // Ensure write succeeded
    }
}
```

  **Explanation**: This test creates multiple readers and one writer using `RwLock` to ensure safe concurrent access to shared data.

### Summary

  Testing threaded applications in Rust involves:

1. **Basic thread execution checks** to ensure threads are running correctly.

2. **Synchronization tests** using `Mutex`, `Arc`, and `RwLock` to avoid race conditions.

3. **Communication tests** using channels to verify inter-thread messaging.

4. **Deadlock prevention checks** to ensure threads do not block each other indefinitely.

5. **Thread pool execution tests** to verify task scheduling and concurrency.

6. **Thread safety tests** to ensure shared data access remains safe and consistent.
   
   These examples demonstrate different testing strategies for concurrent Rust applications. Each example includes a `#[test]` function to illustrate how to write and run tests for multi-threaded code effectively.

### 

### Explanation of Examples:

1. **Basic Threading:** Spawning a single thread that runs a loop.
2. **Thread with Move:** Moving ownership of a vector into a thread using `move`.
3. **Thread Sleep:** Demonstrating thread sleep.
4. **Multiple Threads:** Spawning multiple threads and ensuring they complete using `join`.
5. **Channels for Communication:** Sending a message from one thread to another via channels.
6. **Multiple Values via Channel:** Sending multiple messages from a thread using a channel.
7. **Arc and Mutex:** Sharing and updating state between multiple threads using `Arc` and `Mutex`.
8. **Deadlock Example:** Illustrating potential deadlocks with multiple `Mutex` locks.
9. **Thread Pool Example:** Manually managing multiple threads to simulate a thread pool.
10. **Parallel Map:** Using multiple threads to process and transform data concurrently.

You can run this code, and it will print the output for each example step by step. The combination of `lib.rs` and `main.rs` allows clean separation of the logic and execution.

To provide a comprehensive set of examples on error handling in Rust, I will cover various real-world scenarios ranging from basic error handling to more advanced use cases involving custom error types, conversions, and integrating with popular Rust libraries.

### `lib.rs`

Below is the content of `lib.rs`, which includes 20 examples of error handling in Rust. These examples cover common patterns such as handling errors with `Result`, using `Option`, creating custom errors, converting errors, and working with third-party libraries.

When multiple threads are not joined in a Rust program, the main thread may finish execution before the spawned threads complete their work. This can lead to several issues:

### Key Issues:

1. **Threads may not finish their tasks**:
   
   - If the main thread ends before the spawned threads complete their execution, those threads will be prematurely terminated, and their work may remain incomplete.
   - In Rust, when the main thread finishes, it terminates the entire process, killing all active threads that haven't been joined.

2. **Loss of results**:
   
   - If the spawned threads are performing critical operations (e.g., updating shared data), and you don’t join them, you may lose the results of those operations. This can result in inconsistent or incomplete data.

3. **Unpredictable behavior**:
   
   - Since threads may be interrupted at any time, the program's output becomes unpredictable if the main thread terminates early. You may see partial or no results from the background threads.

### Example:

```rust
use std::thread;

fn main() {
    for i in 0..5 {
        thread::spawn(move || {
            println!("Thread {}", i);
        });
    }
    // If we don't join the threads, the main thread may exit before they finish.
    println!("Main thread is done!");
}
```

### What can happen:

- The main thread prints "Main thread is done!" and exits.
- Some or all of the spawned threads may not have time to print their messages, leading to incomplete or missing output.

### Correct Way (with Join):

By joining the threads, you ensure that the main thread waits for all spawned threads to finish their work:

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Ensures the main thread waits for all spawned threads to finish
    }

    println!("Main thread is done!");
}
```

### Summary:

- **Without `join()`**: Threads may be prematurely terminated, leading to incomplete work and unpredictable behavior.
- **With `join()`**: Ensures that all threads complete their execution before the main thread finishes.

#### `lib.rs`

```rust
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;
use std::{error::Error, fmt};

// Basic error handling with Result
pub fn open_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Handling Option with Result
pub fn parse_optional_number(input: Option<&str>) -> Result<i32, ParseIntError> {
    input.unwrap_or("0").parse::<i32>()
}

// Chaining multiple errors
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Custom error types with Display and Error traits
#[derive(Debug)]
pub struct InvalidInputError {
    details: String,
}

impl InvalidInputError {
    fn new(msg: &str) -> InvalidInputError {
        InvalidInputError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Input: {}", self.details)
    }
}

impl Error for InvalidInputError {}

pub fn validate_number(input: &str) -> Result<i32, InvalidInputError> {
    match input.parse::<i32>() {
        Ok(num) if num > 0 => Ok(num),
        _ => Err(InvalidInputError::new("Only positive numbers are allowed")),
    }
}

// Using the `?` operator with custom errors
pub fn double_positive_number(input: &str) -> Result<i32, InvalidInputError> {
    let number = validate_number(input)?;
    Ok(number * 2)
}

// Combining different error types using `Box<dyn Error>`
pub fn parse_and_double(input: &str) -> Result<i32, Box<dyn Error>> {
    let number = input.parse::<i32>()?;
    if number < 0 {
        return Err(Box::new(InvalidInputError::new("Negative number")));
    }
    Ok(number * 2)
}

// Using Option for a possible None value
pub fn find_word(input: &str, word: &str) -> Option<usize> {
    input.find(word)
}

// Error handling with external libraries (reqwest example)
#[cfg(feature = "reqwest")]
pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// Handling JSON parsing errors (serde_json example)
#[cfg(feature = "serde_json")]
pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(input)
}

// Using Result with Option to avoid nested Result<Option<T>>
pub fn parse_number_and_check(input: &str) -> Result<Option<i32>, ParseIntError> {
    match input.parse::<i32>() {
        Ok(n) => Ok(Some(n)),
        Err(e) => Err(e),
    }
}

// Wrapping errors for more context
pub fn open_file_with_context(file_path: &str) -> Result<String, io::Error> {
    File::open(file_path).map_err(|e| io::Error::new(e.kind(), format!("Failed to open {}: {}", file_path, e)))
}

// Ignoring errors
pub fn log_error_to_file(error: &str) {
    let _ = File::create("error.log").and_then(|mut f| f.write_all(error.as_bytes()));
}

// Error handling in iterators
pub fn parse_numbers(inputs: Vec<&str>) -> Vec<Result<i32, ParseIntError>> {
    inputs.into_iter().map(|s| s.parse::<i32>()).collect()
}

// Using unwrap_or_else for custom error handling
pub fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
}

// Working with Result and ? operator in functions that return Option
pub fn maybe_parse_number(input: &str) -> Option<Result<i32, ParseIntError>> {
    Some(input.parse::<i32>())
}

// Handling errors with an iterator that returns Option
pub fn find_first_invalid_number(inputs: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    inputs.into_iter().find_map(|input| match input.parse::<i32>() {
        Ok(_) => None,
        Err(e) => Some(Err(e)),
    })
}

// Returning custom error types from functions
pub fn parse_and_validate(input: &str) -> Result<i32, InvalidInputError> {
    let number = input.parse::<i32>().map_err(|_| InvalidInputError::new("Parse error"))?;
    if number < 0 {
        return Err(InvalidInputError::new("Negative number"));
    }
    Ok(number)
}

// Using unwrap_or for Option types
pub fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

// Propagating errors from lower-level functions
pub fn read_file_and_double_numbers(file_path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let content = open_file(file_path)?;
    content
        .lines()
        .map(|line| line.parse::<i32>().map_err(|e| e.into()))
        .collect()
}

// Example of combining different error types using `thiserror` (optional)
#[cfg(feature = "thiserror")]
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}
```

### `tests.rs`

Below is the `tests.rs` file containing tests for each function in `lib.rs`. This will ensure that the error handling behaves as expected.

#### `tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file() {
        let result = open_file("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_optional_number() {
        assert_eq!(parse_optional_number(Some("42")).unwrap(), 42);
        assert_eq!(parse_optional_number(None).unwrap(), 0);
    }

    #[test]
    fn test_validate_number() {
        assert!(validate_number("10").is_ok());
        assert!(validate_number("-10").is_err());
    }

    #[test]
    fn test_double_positive_number() {
        assert_eq!(double_positive_number("10").unwrap(), 20);
        assert!(double_positive_number("-10").is_err());
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("10").unwrap(), 20);
        assert!(parse_and_double("-10").is_err());
    }

    #[test]
    fn test_find_word() {
        assert_eq!(find_word("hello world", "world"), Some(6));
        assert_eq!(find_word("hello world", "rust"), None);
    }

    #[test]
    fn test_parse_number_and_check() {
        assert_eq!(parse_number_and_check("42").unwrap(), Some(42));
        assert!(parse_number_and_check("abc").is_err());
    }

    #[test]
    fn test_open_file_with_context() {
        let result = open_file_with_context("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_numbers() {
        let inputs = vec!["1", "2", "abc"];
        let results = parse_numbers(inputs);
        assert_eq!(results[0], Ok(1));
        assert!(results[2].is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Division by zero is not allowed!")]
    fn test_safe_divide_panic() {
        safe_divide(10, 0);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn test_read_file_and_double_numbers() {
        let result = read_file_and_double_numbers("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_and_validate() {
        assert!(parse_and_validate("42").is_ok());
        assert!(parse_and_validate("abc").is_err());
        assert!(parse_and_validate("-1").is_err());
    }
}
```

### Instructions for Running the Tests

1. **Create the Project Structure**:
   Create a new Rust library project:
   
   ```bash
   cargo new rust_error_handling --lib
   cd
   
   rust_error_handling
   ```

2. **Replace `lib.rs` with Provided Code**:
   Replace the contents of `src/lib.rs` with the code provided above.

3. **Create `tests.rs`**:
   Create a new file in `src/tests.rs` and copy the tests code into it.

4. **Run Tests**:
   Run the tests using Cargo:
   
   ```bash
   cargo test
   ```

### Conclusion

This example provides a comprehensive set of Rust error-handling patterns using `Result`, `Option`, custom error types, error conversions, and interactions with external libraries. The `tests.rs` file ensures that each function's error handling is correct and demonstrates how to write effective tests for error scenarios.



Creating 10,000,000 threads is not feasible due to the high memory consumption and the operating system's limitations on the number of threads that can be spawned. Each thread requires stack space, and spawning too many threads will exhaust system resources, leading to errors like "Cannot allocate memory (os error 12)" and "Resource temporarily unavailable."

### Why This Happens

1. **Memory Usage**: Each thread has a stack size (e.g., 2MB by default on some systems). Creating millions of threads would consume terabytes of virtual memory, which exceeds typical system limits.

2. **OS Limits**: The operating system has a limit on the number of threads a process can create. Exceeding this limit results in an "OS error 11, Resource temporarily unavailable."

### Solution: Use Thread Pools or Asynchronous Programming

Instead of creating millions of threads, consider using a **thread pool** or **asynchronous programming**. Thread pools reuse a fixed number of threads to perform multiple tasks, while asynchronous programming allows you to handle many tasks without creating many threads.

#### Example: Using `rayon` for Thread Pool in Rust

The `rayon` library provides a simple and efficient thread pool. Here’s how you can use it:

```rust
use rayon::prelude::*;

fn main() {
    let data = (0..10_000_000).collect::<Vec<_>>(); // Example vector with 10 million elements

    let results: Vec<_> = data
        .par_iter() // Parallel iterator
        .map(|&num| num * num) // Square each number
        .collect(); // Collect results into a new vector

    println!("Processed {} elements.", results.len());
}
```

#### Explanation

1. **Parallel Iterator (`par_iter`)**: `rayon` provides a `par_iter` method that runs in parallel using a thread pool.
2. **No Thread Explosion**: `rayon` manages a fixed-size thread pool, preventing thread explosion and excessive memory usage.
3. **Efficient and Safe**: This approach is efficient, safe, and easy to use for CPU-bound tasks.

#### Example: Using `tokio` for Asynchronous Programming

For tasks that involve I/O-bound operations or need to handle many tasks efficiently, use `tokio`:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..10_000_000 {
        let handle = task::spawn(async move {
            // Example: perform some async task
            i * i
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let results: Vec<_> = futures::future::join_all(handles).await;

    println!("Processed {} tasks.", results.len());
}
```

### Conclusion

- **Avoid Creating Millions of Threads**: Use thread pools (`rayon`) or asynchronous programming (`tokio`) to handle many tasks efficiently.
- **Manage Resources Efficiently**: Both solutions help manage resources efficiently and prevent issues like "Cannot allocate memory" or "Resource temporarily unavailable."

These approaches provide scalable and robust solutions for handling large-scale concurrency in Rust.

```rust

```
