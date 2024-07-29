#[allow(dead_code)]

use tokio::io::AsyncReadExt;
use tokio::time;
use log::Level;

async fn sleeper() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake");
}

async fn reader() {
    log::info!("Reading file");
    let mut f = tokio::fs::File::open("csv_file.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("size of file read {} in bytes", contents.len());
}

async fn run(){
    // the below will run sync..ly
    // sleeper().await;
    // reader().await;
    // now running the same async..ly
    tokio::join!(
        sleeper(),
        reader(),
    );
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();
    rt.block_on(future);
}