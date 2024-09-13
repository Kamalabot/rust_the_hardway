use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn parallel_map_example() {
    // let data = vec![1, 2, 3, 4, 5];
    let data: Vec<i32> = (0..1000).collect();
    let mut handles = vec![];
    let result = Arc::new(Mutex::new(vec![]));
    // here the empty vector is first created as mutex

    for num in data {
        let result = Arc::clone(&result);
        // mutex is cloned
        let handle = thread::spawn(move || {
            let squared = num * num;
            // observe how the push is done through lock
            result.lock().unwrap().push(squared);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Squared Results: {:?}", *result.lock().unwrap());
}

use rayon::prelude::*;

fn main() {
    let now = Instant::now();
    parallel_map_example();
    let elapsed = now.elapsed();
    println!("Parallel time elapsed w/o rayon: {:?}", elapsed);
    let main_vec: Vec<u32> = (0..1000).collect();

    let again = Instant::now();
    let sqr_vec: Vec<u32> = main_vec.iter().map(|x| x * x).collect();
    let ag_elapsed = again.elapsed();
    println!("Non prarallel time elapsed: {:?}", ag_elapsed);

    let start = Instant::now();
    let results: Vec<u32> = main_vec.par_iter().map(|&num| num * num).collect();
    let elapsed = start.elapsed();
    println!("Parallel time elapsed with Rayon: {:?}", elapsed);
}
