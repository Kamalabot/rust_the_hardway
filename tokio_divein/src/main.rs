#![allow(warnings)]
#![allow(unused_imports)]
use tokio;
use tokio_divein::*;

#[tokio::main]
async fn main() {
    // if not unwrap, then need the main
    // to return Result
    // rw_file().await.unwrap();
    // tcp_listener().await.unwrap();
    // task_spawner().await;
    // tokio_mpsc().await;
    // oneshot().await;
    // multi_tx().await;
    // read_in().await;
    // read_buf().await;
    // recurse_stdout(5).await;
    // http_server().await;
    // barrier_example().await;
    // async_mutex().await;
    useful_semaphore().await;
}
