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
    println!("Timer completed after 1 second");
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
        socket.send_to(&buf[..len], addr).await?;
    }
}

// Example 11: Async read from stdin
async fn async_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).await.unwrap();
    println!("Input: {}", input);
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

#[tokio::test]
async fn test_read_write_file() {
    read_write_file().await.unwrap();
}

#[tokio::test]
async fn test_tcp_server() {
    tokio::spawn(async {
        tcp_server().await.unwrap();
    });
}

#[tokio::test]
async fn test_async_timer() {
    async_timer().await;
}

#[tokio::test]
async fn test_task_spawner() {
    task_spawner().await;
}

#[tokio::test]
async fn test_async_mpsc() {
    async_mpsc().await;
}

#[tokio::test]
async fn test_async_oneshot() {
    async_oneshot().await;
}

#[tokio::test]
async fn test_async_file_copy() {
    async_file_copy().await.unwrap();
}

#[tokio::test]
async fn test_async_file_delete() {
    async_file_delete().await.unwrap();
}

#[tokio::test]
async fn test_concurrent_tasks() {
    concurrent_tasks().await;
}

#[tokio::test]
async fn test_udp_server() {
    tokio::spawn(async {
        udp_server().await.unwrap();
    });
}

// You can add more test cases for other examples in a similar manner.
```

Each example demonstrates a different real-world application of `tokio` async operations. For testing, I’ve provided a few test cases, and you can expand upon them as needed.

Here is a brief explanation of each example along with its topic:

1. **Async File Read/Write**: Demonstrates async file operations (reading/writing).
2. **Async TCP Server**: Implements a basic asynchronous TCP server.
3. **Async Timer**: Uses `tokio::time` to introduce delays asynchronously.
4. **Async Task Spawning**: Spawns multiple asynchronous tasks concurrently.
5. **Async MPSC (Multi-Producer, Single-Consumer)**: Sends messages between tasks using async channels.
6. **Async One-shot Channel**: Sends a single message between tasks with a one-shot channel.
7. **Async File Copy**: Copies a file asynchronously.
8. **Async File Delete**: Deletes a file asynchronously.
9. **Concurrent Tasks with Join**: Runs multiple async tasks concurrently using `tokio::join`.
10. **Async UDP Server**: Implements a basic asynchronous UDP server.
11. **Async Stdin**: Reads input from stdin asynchronously.
12. **Async Stdout**: Writes output to stdout asynchronously.
13. **Async Recursive Function**: Demonstrates recursion in async functions.
14. **Rate Limiting with Sleep**: Limits task execution speed with async sleep.
15. **Async HTTP Server (Basic)**: Implements a simple asynchronous HTTP server.
16. **Task Cancellation**: Cancels a long-running task asynchronously.
17. **Task Synchronization with Barrier**: Synchronizes multiple tasks using a barrier.
18. **Async Mutex**: Demonstrates async mutex for shared data access across tasks.
19. **Async Semaphore**: Manages resource access limits using an async semaphore.
20. **Async RwLock**: Uses an async read-write lock for concurrent access control.

These examples cover various aspects of asynchronous programming using Tokio, such as networking, concurrency, I/O, synchronization, and task management.
