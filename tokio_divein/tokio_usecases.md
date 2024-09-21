Here’s a file with 20 real-world examples of `tokio` async usage, along with corresponding test cases. The examples range from simple to advanced and include topics like async networking, file I/O, task scheduling, concurrency, and more.

```rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task;

// Example 1: Async file read and write
async fn read_write_file() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    file.write_all(b"hello, world!").await?;

    let mut file = File::open("foo.txt").await?;
    let mut buffer = vec![0; 12];
    file.read_exact(&mut buffer).await?;
    Ok(())
}

// Example 2: Async TCP server
async fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 1024];
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            return;
        }
        stream.write_all(&buffer[0..n]).await.unwrap();
    }
}

async fn tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

// Example 3: Async timer
async fn async_timer() {
    time::sleep(Duration::from_secs(1)).await;
    println!("Timer completejd after 1 second");
}

// Example 4: Async task spawning
async fn task_spawner() {
    let task1 = tokio::spawn(async {
        println!("Task 1 is running");
    });

    let task2 = tokio::spawn(async {
        println!("Task 2 is running");
    });

    let _ = tokio::join!(task1, task2);
}

// Example 5: Async channel communication (MPSC)
async fn async_mpsc() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send(42).await.unwrap();
    });

    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}

// Example 6: Async one-shot channel
async fn async_oneshot() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Hello").unwrap();
    });

    let msg = rx.await.unwrap();
    println!("Received: {}", msg);
}

// Example 7: Async file copy
async fn async_file_copy() -> io::Result<()> {
    tokio::fs::copy("src.txt", "dst.txt").await?;
    Ok(())
}

// Example 8: Async file delete
async fn async_file_delete() -> io::Result<()> {
    tokio::fs::remove_file("dst.txt").await?;
    Ok(())
}

// Example 9: Concurrent tasks with join
async fn concurrent_tasks() {
    let task1 = tokio::spawn(async {
        println!("Task 1 is running");
    });

    let task2 = tokio::spawn(async {
        println!("Task 2 is running");
    });

    let (res1, res2) = tokio::join!(task1, task2);
    res1.unwrap();
    res2.unwrap();
}

// Example 10: Async UDP server
async fn udp_server() -> io::Result<()> {
    let socket = tokio::net::UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        socket.send_to    (&buf[..len], addr).await?;
    }
}

// Example 11: Async read from stdin
async fn async_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).await.unwrap();
    println!("Input: {}", input);
}
//The issue arises because `tokio::io::stdin()` does not behave interactively as expected. It reads the entire stdin input until EOF, which is usually signaled by pressing `Ctrl + D` on Unix or `Ctrl + Z` on Windows. This behavior is typical for `read_to_string` and not suitable for interactive line-by-line input.

//To achieve line-by-line input asynchronously (reading and printing after pressing enter), you can use **`tokio::io::AsyncBufReadExt::read_line`** with a buffered reader, such as `tokio::io::BufReader`. This allows you to read input as soon as the user presses enter, without blocking.

use tokio::io::{self, AsyncBufReadExt, BufReader};

async fn async_stdin() {
    let mut input = String::new();

    // Create a buffered reader for async stdin
    let mut stdin = BufReader::new(io::stdin());

    // Read a line asynchronously
    stdin.read_line(&mut input).await.unwrap();
    println!("Input: {}", input);
}

#[tokio::main]
async fn main() {
    println!("Please enter some input:");
    async_stdin().await;
}
// The program will wait for the user to press 
// enter after typing input, and it will print
// the input immediately. It no longer waits 
// for EOF, so it behaves more interactively.

//With this setup, the CLI will accept input when
// enter is pressed and print the input as expected
// without blocking.

// below setup uses while loop to continue getting 
// data from the stdin, and buffer it
use tokio::io::{self, AsyncBufReadExt, BufReader};

async fn async_stdin() {
    let mut input = String::new();

    // Create a buffered reader for async stdin
    let mut stdin = BufReader::new(io::stdin());

    loop {
        input.clear(); // Clear the input buffer for new input

        // Prompt for input
        println!("Please enter some input (type 'exit' to quit):");

        // Read a line asynchronously
        match stdin.read_line(&mut input).await {
            Ok(0) => break, // EOF reached, exit the loop
            Ok(_) => {
                // Check for exit command
                if input.trim() == "exit" {
                    println!("Exiting...");
                    break;
                }
                println!("Input: {}", input);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    async_stdin().await;
}

// Example 12: Async write to stdout
async fn async_stdout() {
    io::stdout().write_all(b"Hello, world!").await.unwrap();
}

// Example 13: Async recursive function
async fn async_recursive(n: u32) {
    if n == 0 {
        return;
    }
    async_recursive(n - 1).await;
}

// Example 14: Rate-limiting with sleep
async fn rate_limiter() {
    for _ in 0..5 {
        println!("Action");
        time::sleep(Duration::from_millis(500)).await;
    }
}

// Example 15: Async HTTP server (basic)
async fn async_http_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
            socket.write_all(response).await.unwrap();
        });
    }
}

// Example 16: Async task cancellation
async fn cancellable_task() -> io::Result<()> {
    let handle = tokio::spawn(async {
        time::sleep(Duration::from_secs(5)).await;
        println!("Task completed");
    });

    time::sleep(Duration::from_secs(1)).await;
    handle.abort();
    Ok(())
}

// Example 17: Task synchronization with barriers
async fn barrier_example() {
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(2));
    let c = barrier.clone();

    tokio::spawn(async move {
        println!("Task 1 waiting");
        c.wait().await;
        println!("Task 1 proceeding");
    });

    println!("Task 2 waiting");
    barrier.wait().await;
    println!("Task 2 proceeding");
}

// Example 18: Async mutex
async fn async_mutex() {
    let data = std::sync::Arc::new(tokio::sync::Mutex::new(0));
    let data1 = data.clone();
    let data2 = data.clone();

    tokio::spawn(async move {
        let mut lock = data1.lock().await;
        *lock += 1;
    });

    let mut lock = data2.lock().await;
    *lock += 1;
}

// Example 19: Async semaphore
async fn async_semaphore() {
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
    let permit = semaphore.acquire().await.unwrap();
    println!("Acquired semaphore permit");
    drop(permit);
}
// more useful semaphore
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

// Simulates a heavy operation, like a database query
async fn perform_heavy_operation(id: usize) {
    println!("Task {} started.", id);
    // Simulating a delay for the heavy operation
    sleep(Duration::from_secs(2)).await;
    println!("Task {} completed.", id);
}

#[tokio::main]
async fn main() {
    // Create a semaphore allowing up to 3 concurrent operations
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    // Simulate 10 incoming tasks
    for i in 0..10 {
        let semaphore_clone = Arc::clone(&semaphore);
        // Each task tries to acquire a permit before proceeding
        let handle = tokio::spawn(async move {
            // Acquire a permit to limit the number of concurrent executions
            let permit = semaphore_clone.acquire().await.unwrap();
            perform_heavy_operation(i).await;
            // Permit is automatically released when dropped here
            drop(permit);
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks completed.");
}

// Rate Limiting: Control access to resources like APIs or database connections, avoiding overload.
// Resource Management: Manage limited resources like network connections, file handles, or thread pools.
// Task Throttling: Limit the number of concurrent jobs in scenarios like data processing, avoiding spikes in resource usage.
// Example 20: Async read/write lock
async fn async_rwlock() {
    let lock = std::sync::Arc::new(tokio::sync::RwLock::new(0));
    let lock1 = lock.clone();
    let lock2 = lock.clone();

    tokio::spawn(async move {
        let read_lock = lock1.read().await;
        println!("Read lock value: {}", *read_lock);
    });

    let mut write_lock = lock2.write().await;
    *write_lock += 1;
}


// Multiple Readers: It allows multiple readers to access the data simultaneously, which is efficient when you need to perform many read operations without modifying the data.
// Single Writer: Only one writer can access the data at a time, and it has exclusive access, meaning no other readers or writers can access the data while a write is occurring.


```

### 1. **Barriers**

- **Concept**: A **barrier** is a synchronization primitive used to block a group of tasks or threads until all of them have reached a certain point. Once all participants have "arrived," they can proceed.
- **Use Case**: Barriers are useful when you need to coordinate multiple tasks to reach the same point before moving forward. For example, when multiple workers need to complete their work before proceeding to the next phase of a program.

### 2. **Async Mutex**

- **Concept**: An **async mutex** is a synchronization primitive that provides exclusive access to a shared resource in an asynchronous context. It allows only one task to access the resource at a time, ensuring that access is mutually exclusive, but doesn't block the thread—it allows other tasks to run while waiting for the lock.
- **Use Case**: Async mutexes are used when you need to protect shared data (like a global counter or a shared data structure) in an async environment, ensuring that only one task modifies the resource at a time without blocking other tasks from running.

### 3. **Locks**

- **Concept**: A **lock** is a general term for any mechanism that allows tasks to synchronize access to a shared resource. It can refer to either a **synchronous** lock (blocking) or an **asynchronous** lock (non-blocking). In an async context, locks such as an async mutex or read-write locks ensure that tasks can coordinate access to resources without blocking the entire thread.
- **Use Case**: Locks are used to prevent race conditions when multiple tasks need to read or modify shared data, ensuring that only one task holds the lock at a time. Async locks are particularly useful for non-blocking applications like servers or concurrent data processing.
- ### Purpose of `RwLock`
  1. **Concurrent Read Access**: `RwLock` is used when you need high concurrency for read-heavy workloads, where reads are frequent, and writes are relatively rare.
  2. **Exclusive Write Access**: Ensures that data is not modified while being read, providing safety by preventing data races during writes.
  
  ### Practical Use Cases
  
  - **Caching Systems**: Read-heavy applications like caching where data is read frequently and occasionally updated.
  - **Configuration Management**: Where a shared configuration might be read often but updated only occasionally.
  - **Resource Management**: Managing access to shared resources that are read often but need to be updated periodically.

### 4. **Semaphore**

- **Concept**: A **semaphore** is a signaling mechanism used to control access to a limited number of resources. It allows a set number of tasks to access the resource concurrently (e.g., a semaphore with a count of 3 allows up to 3 tasks to run in parallel). When the semaphore count is exhausted, tasks must wait until a resource becomes available.
- **Use Case**: Semaphores are used when you want to limit the number of concurrent accesses to a resource. For example, controlling access to a database connection pool or managing the number of concurrent requests in a server to prevent overload.

These primitives are key for managing synchronization and safe access to resources in asynchronous environments, particularly in scenarios where multiple tasks are working concurrently but need controlled access to shared data or limited resources.

Each example demonstrates a different real-world application of `tokio` async operations. For testing, I’ve provided a few test cases, and you can expand upon them as needed.

To understand how `async` and `await` affect the examples, let's look at a few scenarios where these keywords control concurrency and non-blocking behavior. Here’s how `async` and `await` work in real-world examples:

### Key Concepts of `async` and `await`:

- **`async` functions** are non-blocking: They return a `Future` immediately when called but do not execute their body until `await` is called on the future.
- **`await`** pauses the current task until the awaited future completes, allowing other tasks to run concurrently.

### Example Breakdown

1. **Async File Read/Write**
   
   ```rust
   async fn read_write_file() -> io::Result<()> {
       let mut file = File::create("foo.txt").await?; // Non-blocking: returns a Future immediately
       file.write_all(b"hello, world!").await?; // Awaits file writing
   
       let mut file = File::open("foo.txt").await?; // Non-blocking open
       let mut buffer = vec![0; 12];
       file.read_exact(&mut buffer).await?; // Awaits until file is read
       Ok(())
   }
   ```
   
   - The `await` on file operations ensures that the task is suspended while the I/O operation is being performed, freeing the runtime to handle other tasks.

2. **Async TCP Server**
   
   ```rust
   async fn handle_client(mut stream: TcpStream) {
       let mut buffer = vec![0; 1024];
       while let Ok(n) = stream.read(&mut buffer).await { // Non-blocking read from client
           if n == 0 {
               return; // Client disconnected
           }
           stream.write_all(&buffer[0..n]).await.unwrap(); // Non-blocking write
       }
   }
   
   async fn tcp_server() -> io::Result<()> {
       let listener = TcpListener::bind("127.0.0.1:8080").await?; // Non-blocking bind to port
       loop {
           let (socket, _) = listener.accept().await?; // Non-blocking accept connection
           tokio::spawn(handle_client(socket)); // Spawn a new task for each client
       }
   }
   ```
   
   - `await` on `TcpStream::read` and `TcpStream::write` ensures that the server doesn't block waiting for I/O. Each client runs in its own task, which allows the server to handle multiple connections concurrently.

3. **Async Timer**
   
   ```rust
   async fn async_timer() {
       time::sleep(Duration::from_secs(1)).await; // Non-blocking sleep
       println!("Timer completed after 1 second");
   }
   ```
   
   - `await` on `time::sleep` makes the timer non-blocking. The task sleeps asynchronously, allowing other tasks to execute while waiting for the timer to complete.

4. **Task Spawning**
   
   ```rust
   async fn task_spawner() {
       let task1 = tokio::spawn(async {
           println!("Task 1 is running");
       });
   
       let task2 = tokio::spawn(async {
           println!("Task 2 is running");
       });
   
       let _ = tokio::join!(task1, task2); // Concurrently awaits both tasks
   }
   ```
   
   - `tokio::spawn` creates new asynchronous tasks. `tokio::join!` ensures both tasks run concurrently, and `await` ensures the main function does not proceed until both tasks complete.

5. **Async MPSC (Multi-Producer, Single-Consumer)**
   
   ```rust
   async fn async_mpsc() {
       let (tx, mut rx) = mpsc::channel(32);
   
       tokio::spawn(async move {
           tx.send(42).await.unwrap(); // Non-blocking send
       });
   
       while let Some(value) = rx.recv().await { // Non-blocking receive
           println!("Received: {}", value);
       }
   }
   ```
   
   - `await` on `tx.send` and `rx.recv` allows message passing between tasks without blocking the runtime, so other tasks can run while waiting for messages.

6. **Concurrent Tasks with Join**
   
   ```rust
   async fn concurrent_tasks() {
       let task1 = tokio::spawn(async {
           println!("Task 1 is running");
       });
   
       let task2 = tokio::spawn(async {
           println!("Task 2 is running");
       });
   
       let (res1, res2) = tokio::join!(task1, task2); // Awaits both tasks concurrently
       res1.unwrap();
       res2.unwrap();
   }
   ```
   
   - `tokio::join!` runs both tasks concurrently. `await` on each ensures that the result is returned only when both tasks have completed, while the runtime remains non-blocked.

### Impact of `async` and `await`

1. **Concurrency**: By marking functions with `async`, we enable them to be run concurrently with other asynchronous tasks, without blocking the execution of other code.
2. **Non-blocking I/O**: In file I/O, networking, or any other potentially long-running operation, `await` prevents the current task from blocking the thread while waiting for an operation to complete. Other tasks can progress during that time.
3. **Task Spawning and Parallelism**: `tokio::spawn` allows us to create lightweight tasks that run in parallel. `await` ensures that the function suspends execution and yields control to the Tokio runtime while waiting for asynchronous operations to complete.

### Visualization of Impact

- Without `async/await`, if you performed an operation like reading from a file or waiting for a network socket, your program would block, meaning the whole system would be stuck until that operation completed.
- With `async/await`, tasks that involve waiting (e.g., I/O operations, timers) don't block the system. Other tasks can run in parallel while waiting, improving performance and resource usage.

### Conclusion

`async` makes functions return a `Future`, and `await` suspends them until the awaited `Future` completes. This combination allows for non-blocking, concurrent execution, which is especially useful for I/O-bound and timer-bound operations, improving both performance and responsiveness in real-world applications.

The key difference between `oneshot::channel` and `mpsc::channel` in Rust lies in the number of senders and the nature of the message passing:

### 1. **`oneshot::channel`**

- **Purpose**: A channel that allows exactly **one** message to be sent from a single sender to a single receiver.

- **Single Send-Receive Pair**: There is **one sender** and **one receiver**. After the sender sends a message, the channel is considered "used up" and can no longer be reused.

- **Usage**: Ideal for situations where you need to send a single result from a background task, often used for signaling completion or returning a result.

- **Unidirectional**: It is a point-to-point channel where only one value can be sent and received.
  
  Example:
  
  ```rust
  use tokio::sync::oneshot;
  
  async fn run_task() {
    let (tx, rx) = oneshot::channel();
  
    // Sender task
    tokio::spawn(async move {
        let _ = tx.send("Hello, World!"); // Send a single message
    });
  
    // Receiver awaits message
    let message = rx.await.unwrap();
    println!("Received: {}", message);
  }
  ```

- The channel completes after sending a single message.

### 2. **`mpsc::channel`**

- **Purpose**: A **multi-producer, single-consumer** channel that allows **multiple senders** to send messages to a single receiver.

- **Multiple Senders**: The key feature is the ability to have multiple producers (senders) sending multiple messages to a single consumer (receiver).

- **Usage**: Useful for distributing tasks to a worker, aggregating results from multiple tasks, or broadcasting events.

- **Buffered or Unbuffered**: It can be either buffered (allowing multiple messages to be in the queue) or unbuffered (blocking until the receiver receives each message).
  
  Example:
  
  ```rust
  use tokio::sync::mpsc;
  use tokio::time::{sleep, Duration};
  
  async fn run_mpsc_example() {
    let (tx, mut rx) = mpsc::channel(10); 
      // Buffered channel with capacity 10
  
    // Spawn multiple sender tasks
    for i in 0..3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let _ = tx_clone.send(format!("Message {}", i)).await;
            sleep(Duration::from_millis(100)).await;
        });
    }
  
    // Close the original sender so the receiver knows no more messages will come
    drop(tx);
  
    // Receiver task
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
  }
  ```

- The receiver can receive multiple messages sent by various senders.

### Summary of Differences:

| Feature                | `oneshot::channel`                          | `mpsc::channel`                             |
| ---------------------- | ------------------------------------------- | ------------------------------------------- |
| **Number of Senders**  | Exactly **one** sender                      | Multiple senders                            |
| **Number of Messages** | Exactly **one** message                     | Multiple messages                           |
| **Use Case**           | Single message signaling or task completion | Task queueing, broadcasting, or aggregation |
| **Uni/Bi-directional** | Unidirectional, one-time use                | Unidirectional, reusable                    |
| **Concurrency**        | Point-to-point communication                | Multi-producer, single-consumer             |

In summary:

- **`oneshot::channel`** is useful for one-off messages, typically used to send a result or signal the completion of a task.
- **`mpsc::channel`** is useful for sending multiple messages from multiple tasks to a single consumer, suitable for task distribution or event aggregation.

In Rust, `async fn` generates a state machine, and recursive `async fn` calls are not allowed without introducing indirection, because the size of the future would be unbounded. To address this issue, you need to **box** the recursive call to ensure the future has a fixed size.

You can fix this by using `Box::pin` for the recursive call, which effectively wraps the future in a heap-allocated `Pin<Box<dyn Future>>` to resolve the recursion issue.

Here's the corrected version of your code:

### Corrected Recursive Async Function Using `Box::pin`

```rust
use tokio::io::{self, AsyncWriteExt};
use std::pin::Pin;
use std::future::Future;

pub async fn recurse_stdout(n: i32) {
    if n == 0 {
        io::stdout().write_all(b"Hello there...\n").await.unwrap();
        return;
    }

    // Format the message and convert it to a byte slice
    let message = format!("Message from {} recursion\n", n);
    io::stdout().write_all(message.as_bytes()).await.unwrap();

    // To handle recursion, box the future to avoid infinite size
    let fut: Pin<Box<dyn Future<Output = ()>>> = Box::pin(recurse_stdout(n - 1));
    fut.await;
}

#[tokio::main]
async fn main() {
    recurse_stdout(5).await;
}
```

### Key Changes:

1. **Boxing the Recursive Future**: The recursive call `recurse_stdout(n - 1)` is now boxed using `Box::pin`, which ensures that each recursive step returns a future of a fixed size, preventing the infinite-size future compilation error.
   
   ```rust
   let fut: Pin<Box<dyn Future<Output = ()>>> = Box::pin(recurse_stdout(n - 1));
   fut.await;
   ```

2. **Fixed-size Future**: Since Rust requires the size of `async fn` futures to be known at compile time, the boxing ensures that each call to `recurse_stdout` is heap-allocated and can be awaited without causing the infinite-size issue.

### How It Works:

- Each recursive call is wrapped inside a `Pin<Box<dyn Future>>`. This makes sure that the size of the future is well-defined (a `Box` is always of a fixed size).
- The recursive `async` function can now safely recurse, printing the messages asynchronously without hitting the infinite-size future problem.

Now your recursive async function will compile and execute properly!

The provided code does not actually lead to a traditional race condition, thanks to Rust's safety mechanisms and `tokio::sync::Mutex`. However, there is a subtle timing issue that may cause confusion when interpreting the results, particularly regarding how locks are handled.

### Understanding the Code Execution:

1. **Shared Data**: `data` is an `Arc` (atomic reference count) around a `tokio::sync::Mutex`, allowing it to be shared safely between multiple asynchronous tasks.

2. **Locking and Modification**:
   
   - The first task (`task`) locks `data1`, increments its value, and then releases the lock when the `task` completes.
   - Meanwhile, `data2` is locked in the main function, incremented, and printed after the task has completed.

3. **Potential Issue**: The value printed by `println!("Outlock is {}", lock);` might not reflect the combined changes made by both tasks, depending on how `await` resolves and the order of locking/unlocking. But, crucially, this is not a race condition; it's the result of how `await` resolves and releases locks.

### Correct Execution with Comments:

Here's a slightly modified version with comments explaining what's happening:

```rust
pub async fn async_mutex() {
    let data = std::sync::Arc::new(tokio::sync::Mutex::new(0));
    let data1 = data.clone();
    let data2 = data.clone();

    // Spawn an asynchronous task to modify the data
    let task = tokio::spawn(async move {
        let mut lock = data1.lock().await; // Locks data1 (Mutex)
        *lock += 1; // Increments the value by 1
        // Lock is automatically released here when going out of scope
    });

    let mut lock = data2.lock().await; // Locks data2 (Mutex) in the main async function
    *lock += 1; // Increments the value by 1

    // Await the spawned task to ensure it completes before printing
    task.await.unwrap();

    // This value might not reflect the task changes immediately since we're printing the lock from the main task's context
    println!("Outlock is {}", lock);

    // Prints the current state of the data in the Mutex (already unlocked)
    println!("Data is {:?}", data);
}
```

### Key Points:

1. **Order of Execution**: When you `await task.await.unwrap();`, the task completes, but because `lock` was already locked by `data2` at this point, it doesn't see the increment performed inside the task immediately until you access `data` again after releasing the lock.

2. **Locks are Safe**: `tokio::sync::Mutex` safely ensures no data race occurs by locking access to the underlying data, preventing simultaneous mutations.

### Conclusion:

There is no data race, but the behavior of printing `lock` might mislead you into thinking otherwise because it's happening in sequence and not reflecting both increments at that exact point of the `println!`. To ensure clarity in concurrent environments, always print and access shared data when the full intended changes are assured to be complete across all contexts.
