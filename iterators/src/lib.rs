pub fn basic_iterator() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    println!("First element : {:?}", iter.next());
    println!("First element : {:?}", iter.next().unwrap());
}

pub fn cons_iterator() {
    let v = vec![7, 8, 9];
    let coniter = v.iter();
    for vec in coniter {
        println!("Elements are: {}", vec);
    }
    let b_iter = v.iter();
    // moment iter is used, the value is moved
    for &vec in b_iter {
        println!("Elements are referred: {}", vec);
    }
    println!("Can still print...thot cannot: {:?}", v);
    // println!("Can print iterator? {:?}", coniter);
    // println!("Can print b_iter? {:?}", b_iter);
}

pub fn collect_example() {
    let v = vec![1, 2, 3];

    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("collected and squared: {:?}", squared);
}

pub fn collect_filter() {
    let v = [1, 2, 3, 4];
    let even: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("The even numbers are: {:?}", even);
}

pub fn enumerati() {
    let v = vec!["a", "y", "a", "k", "a"];
    for (idx, val) in v.iter().enumerate() {
        println!("val: {}, idx: {}", val, idx);
    }
}

pub fn paired() {
    let name = vec!["boc", "tob", "elm"];
    let age = vec![10, 25, 78];

    let paired: Vec<(&str, i32)> = name
        .iter()
        .zip(age.iter())
        .map(|(&name, &age)| (name, age))
        .collect();
    println!("Paired: {:?}", paired);
}

pub fn any_all() {
    let score = vec![1, 2, 5, 7, 9, 10];
    let has_even = score.iter().any(|&x| x % 2 == 0);
    println!("Has even: {:?}", has_even);
    let all_even = score.iter().all(|&x| x % 2 == 0);
    println!("Has even: {:?}", all_even);
}

pub fn find_ex(invec: &[i32]) {
    // learnt a bit more in using slice instead of vec!
    let fnd = invec.iter().find(|&&x| x == 3).unwrap();
    println!("Found: {:?}", fnd);
    let pos = invec.iter().position(|&x| x == 4);
    println!("Position: {:?}", pos);
    let acu = invec.iter().fold(0, |acc, &f| acc + f);
    println!("Acumulated Sum: {}", acu);
}

pub fn chain_flat(v1: &[i32], v2: &[i32], pv: Vec<Vec<i32>>) {
    let chained: Vec<i32> = v1.iter().chain(v2.iter()).cloned().collect();
    println!("Chained: {:?}", chained);
    let flatty: Vec<i32> = pv.into_iter().flat_map(|x| x.into_iter()).collect();
    println!("flattened vector: {:?}", flatty);
    let use_iter: Vec<i32> = flatty
        .iter()
        .inspect(|&x| print!("{:?} -> ", x))
        .map(|&x| x * x)
        .collect();
    println!();
    println!("Used iter: {:?}", use_iter);
    println!("Infinite Iterators:");
    let result_inf: Vec<i32> = (1..).take(5).collect();
    println!("using inf_iterator: {:?}", result_inf);

    let skipped: Vec<i32> = flatty.into_iter().skip(2).collect();
    println!("{:?}", skipped);
    let steped: Vec<i32> = chained.into_iter().step_by(2).collect();
    println!("{:?}", steped);

    let v = vec!["1", "a", "3", "b"];
    let result: Vec<i32> = v.iter().filter_map(|x| x.parse::<i32>().ok()).collect();
    println!("Filtered and Mapped: {:?}", result);
}

pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

use std::iter::Iterator;

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn custom_iterator_example() {
    let counter = Counter::new();
    for val in counter {
        println!("Custom Iterator: {}", val);
    }
}

pub fn complex_chaining_example() {
    let v = vec![1, 2, 3, 4, 5];
    let result = v
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .fold(0, |acc, x| acc + x);
    println!("Complex Chaining Result: {}", result); // 20
}
