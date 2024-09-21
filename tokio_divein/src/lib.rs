// async immediately returns a future but
// waits for await to be called for executing
// the body
//
// await pauses the current task until the
// awaited future completes, allowing other
// tasks to run

// Async file Read / Write
// We need to use tokio's fs, and io
#![allow(unused_imports)]
#![allow(warnings)]

use tokio::fs::File;
use tokio::io::{self, stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
// above import required for creating, Rw to file
use std::str::from_utf8;
// required for converting
// use std::time::Instant;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio::time::{self, sleep, Duration, Instant};

// create file and write_all some content to it
// create vector buffer and read it back
// convert the read buffer into string using from_utf8
// write a tcp handler and listener
// spawn tasks inside tokio, and run loops with different wait times
// spawn oneshot channel and send data
// spawn multi-sender single reciever channel
//
pub async fn rw_file() -> io::Result<()> {
    let mut file = File::create("async.foo").await?;
    file.write_all(b"Hey Rusty...").await?;
    let mut file = File::open("async.foo").await?;
    let mut buffer = vec![0; 12];
    // this is interesting way to read buffer
    file.read_exact(&mut buffer).await?;
    println!("read buffer: {:?}", buffer);
    // output is just numbers
    let str_buf = from_utf8(&buffer).unwrap();
    println!("String output: {}", str_buf);
    Ok(())
}

pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 1024];
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            return;
        }
        stream.write_all(&buffer[0..n]).await.unwrap();
        let mut buf_str = from_utf8(&buffer).unwrap();
        println!("Writing output: {}", buf_str);
    }
}

pub async fn tcp_listener() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket));
    }
}

pub async fn http_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6055").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Got connected...");
            let resp = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            socket.write_all(resp).await.unwrap();
        });
    }
}

pub async fn udp_server() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:6006").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        socket.send_to(&buf[..len], addr).await;
    }
}

pub async fn read_in() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).await.unwrap();
    println!("Input is: {}", input);
    // need to press ctrl+D to get the outpuut
}

pub async fn read_buf() {
    // will close after one input enter
    let mut input = String::new();
    // creating a buffer with the io::stdin()
    let mut stdinbuf = BufReader::new(stdin());
    stdinbuf.read_line(&mut input).await.unwrap();
    println!("Input line: {}", input);
}
// You can fix this by using Box::pin for the recursive call,
// which effectively wraps the future in a heap-allocated
// Pin<Box<dyn Future>> to resolve the recursion issue.
use std::future::Future;
use std::pin::Pin;

pub async fn recurse_stdout(n: i32) {
    if n == 0 {
        io::stdout().write_all(b"Hello there...\n").await.unwrap();
        return;
    }
    // move down every call
    let byt_str = format!("Message from {} recursion \n", n);
    io::stdout().write_all(byt_str.as_bytes()).await.unwrap();
    let fut: Pin<Box<dyn Future<Output = ()>>> = Box::pin(recurse_stdout(n - 1));
    fut.await;
}

pub async fn task_spawner() {
    let ins = Instant::now();
    // Timing the execution
    let task1 = tokio::spawn(async {
        for _ in 0..5 {
            time::sleep(Duration::from_secs(5)).await;
            println!("Task1 is running... with 5 sec wait");
        }
    });
    let task2 = tokio::spawn(async {
        for _ in 0..3 {
            time::sleep(Duration::from_secs(2)).await;
            println!("Task2 is running...with 2 sec wait");
        }
    });
    let _ = tokio::join!(task1, task2);
    // get the elapsed time
    println!("Elapsed instance: {:?}", ins.elapsed())
}

pub async fn cancel_task() -> io::Result<()> {
    let handle = tokio::spawn(async {
        time::sleep(Duration::from_secs(5)).await;
        println!("Task Completed")
    });
    time::sleep(Duration::from_secs(2)).await;
    handle.abort();
    Ok(())
}

pub async fn tokio_mpsc() {
    let (tx, mut rx) = mpsc::channel(32);
    let vek_num = &[1..10];
    tokio::spawn(async move {
        // we bring the tx and vek_num into the closure
        for i in 1..10 {
            tx.send(i * 2).await.unwrap();
        }
        println!("Vec_num: {:?}", vek_num);
    });

    while let Some(val) = rx.recv().await {
        println!("Recieved: {}", val);
    }
}

pub async fn multi_tx() {
    let (tx, mut rx) = mpsc::channel(10);
    for i in 0..300 {
        let tx_c = tx.clone(); // Cloning senders
        tokio::spawn(async move {
            sleep(Duration::from_secs(15)).await;
            // the above sleep is awaited but will hold the thread in
            // sleep state
            let _ = tx_c.send(format!("From task spawned in {}", i)).await;
            // all of the 300 threads will hit the recv
        });
    }
    drop(tx); // closing down transfers
    while let Some(x) = rx.recv().await {
        println!("Recieved: {}", x);
    }
}

pub async fn oneshot() {
    let (tx, rx) = oneshot::channel();
    let yuf = "New Span";
    tokio::spawn(async move { tx.send(yuf).unwrap() });
    let recpt = rx.await.unwrap();
    println!("Recd: {}", recpt);
}

pub async fn barrier_example() {
    let bar = std::sync::Arc::new(tokio::sync::Barrier::new(2));
    let b = bar.clone();
    // let d = bar.clone();
    tokio::spawn(async move {
        println!("Task 1 waiting");
        b.wait().await;
        // expecting this wait will take
        // it to c.wait()
        println!("task1 is proceeding");
    });
    let c = bar.clone();
    tokio::spawn(async move {
        println!("Task c is waiting");
        c.wait().await;
        println!("task c is through");
    });
    // hits the below println
    println!("Main Task waiting...");
    // enters wait mode
    bar.wait().await; // from here how did
                      // it go up to b
    println!("Task 2 proceeding in main");
}

pub async fn async_mutex() {
    let data = std::sync::Arc::new(tokio::sync::Mutex::new(0));
    let data1 = data.clone();
    let data2 = data.clone();

    let task = tokio::spawn(async move {
        let mut lock = data1.lock().await;
        *lock += 1;
        println!("lock inside task1: {}", lock);
    });

    let mut lock = data2.lock().await;
    // *outlock += 1;
    task.await.unwrap();
    *lock += 1;
    // this wait for the above body to execute
    println!("Outlock is {}", lock);
    println!("data is {:?}", data);
}

pub async fn semph() {
    let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
    let permit = sem.acquire().await.unwrap();
    println!("Acquired permit");
    drop(permit)
}

pub async fn heavy_2sec(id: usize) {
    println!("starting task: {}", id);
    sleep(Duration::from_secs(2)).await;
    println!("finishing task: {}", id);
}

pub async fn useful_semaphore() {
    let semp = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
    let mut handles = vec![];

    for i in 0..10 {
        let sem_clone = std::sync::Arc::clone(&semp);
        // get permit for each task
        let handle = tokio::spawn(async move {
            let permit = sem_clone.acquire().await.unwrap();
            heavy_2sec(i).await;
            drop(permit);
        });
        handles.push(handle);
    }
    for n in handles {
        n.await.unwrap();
    }
    println!("Tasks done...");
}
pub async fn async_rwlock() {
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

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // Shared data protected by an RwLock
    let data = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    // Spawn 5 reader threads
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_lock = data_clone.read().unwrap();
            println!("Reader {} reads: {}", i, *read_lock);
            // Lock is released when read_lock goes out of scope
        });
        handles.push(handle);
    }

    // Spawn 1 writer thread
    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut write_lock = data_clone.write().unwrap();
        *write_lock += 10;
        println!("Writer updated the value to: {}", *write_lock);
        // Lock is released when write_lock goes out of scope
    });
    handles.push(writer_handle);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Final read to check updated value
    let final_value = data.read().unwrap();
    println!("Final value is: {}", *final_value);
}
