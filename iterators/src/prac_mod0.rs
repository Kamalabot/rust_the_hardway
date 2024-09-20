// pull all the activities in single func
// call that in in the main and run it
// the function will do the printing internally
// * use iter() and next()
// * use for loop on the iterator
// * use collect with map
// * use collect with filter, might require to deref
// * use enumerate
// * use all, any, find, position,
// * use zip to create tuple of vectors, including parsing
// * chain two vectors into one
// * flatten a multi-dim vectors
pub fn prac() {
    // use iter() and next()
    let vec1 = vec![1, 6, 7, 9];
    let mut iter_vec1 = vec1.iter();
    println!("{:?}", iter_vec1.next());
    println!("{:?}", iter_vec1.next());
    // println!("{}", true)
    for elm in iter_vec1 {
        // will print the remaining data after
        // above two nexts
        println!("Getting data: {}", elm)
    }
    let sqr_vec: Vec<i32> = vec1.iter().map(|x| x * x).collect();
    println!("Squared vector: {:?}", sqr_vec);
    let f_vec: Vec<i32> = vec1.into_iter().filter(|x| *x > 5).collect();
    println!("vec1 is filtered into f_vec: {:?}", f_vec);
    let byk = vec!["b", "u", "y", "a", "k", "a"];
    for (idx, val) in byk.iter().enumerate() {
        println!("Idx: {}, val: {}", idx, val)
    }
    let v2 = vec!["bot", "ves", "pho"];
    let v3 = vec!["5", "7", "9"];

    let pairet: Vec<(&str, i32)> = v2
        .iter()
        .zip(v3.iter())
        .map(|(&x1, &x3)| (x1, x3.parse::<i32>().unwrap()))
        .collect();
    println!("Paired: {:?}", pairet);
    let v6: Vec<i32> = vec![8, 7, 9, 5, 1, 2, 6, 8, 55, 88];
    let v6_even = v6.iter().any(|&x| x % 2 == 0);
    println!("V6 has even? {}", v6_even);
    let v6_all = v6.iter().all(|&x| x % 2 == 0);
    println!("V6 is all even? {}", v6_all);

    let fnd = v6.iter().find(|&&x| x == 5).unwrap();
    let pos = v6.iter().position(|&x| x == 5).unwrap();
    let acum = v6.iter().fold(10, |acc, &x| acc + x);
    println!("Found 5: {}", fnd);
    println!("Found 5 at : {}", pos);
    println!("Accumulated sum : {}", acum);
    println!("Actual sum : {}", v6.iter().sum::<i32>());
    let v7 = vec![26, 75, 32, 65, 12, 89, 75];
    // when using in the main, consume the vectors
    let chained: Vec<i32> = v6.into_iter().chain(v7.into_iter()).clone().collect();
    println!("Chained vector: {:?}", chained);
    // println!("mostly moved: {:?}", v6)
    let unflat = vec![vec![7, 6], vec![67, 21], vec![56, 32]];
    let flattened: Vec<i32> = unflat.into_iter().flat_map(|x| x.into_iter()).collect();
    println!("Flattened: {:?}", flattened);
}
