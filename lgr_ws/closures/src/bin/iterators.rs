
#[allow(dead_code)]
pub trait Iterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // this is called associative types.
}
#[test]
fn test_iters(){
    let vk1 = vec![6, 5, 2, 6, 7];
    let mut vk_iter = vk1.iter();
    assert_eq!(vk_iter.next(), Some(&6));
    assert_eq!(vk_iter.next(), Some(&5));
    assert_eq!(vk_iter.next(), Some(&2));
    assert_eq!(vk_iter.next(), Some(&6));
    assert_eq!(vk_iter.next(), Some(&7));
    assert_eq!(vk_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe{
    make: String,
    size: i32
}

fn shoe_map_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe>{
    shoes.into_iter().filter(|a| a.size > size).collect()
}

#[test]
fn test_sum(){
    let vk1 = vec![5, 6, 7, 9];
    let vk1_iter = vk1.iter();
    let sum: i32 = vk1_iter.sum();
    assert_eq!(sum, 27);
} 

#[test]
fn test_map() {
    let v1 = vec![5, 6, 9];
    let v1_iter = v1.iter();
    let v2: Vec<i32> = v1_iter.map(|x| x + 5).collect();
    assert_eq!(v2, vec![10, 11, 14]);
}

#[test]
fn test_shoe_map(){
    let sh1 = vec![
        Shoe{
            make:"puma".to_owned(),
            size:5,
        },
        Shoe{
            make:"Argon".to_owned(),
            size:11,
        },
        Shoe{
            make:"Neon".to_owned(),
            size:10,
        },
        Shoe {
            make:"fagro".to_owned(),
            size:9
        }
    ];
    let sh_f: Vec<Shoe> = shoe_map_size(sh1, 9);
    assert_eq!(sh_f, vec![
        Shoe{
            make:"Argon".to_owned(),
            size:11,
        },
        Shoe{
            make:"Neon".to_owned(),
            size:10,
        }
    ])
}

struct Counter{
    count: u32
}

impl Counter{
    fn new() -> Self{
        // Counter {
            // count: 0
        // }
        Self {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32; 

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_other_iterator_methods() {
    let sum = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|a  b| a * b)
        .filter(|x| x % 3 == 0) 
        .sum();
    assert_eq!(10, sum)
}

fn main(){
    let v1 = vec![5, 6, 8];
    let v1_iter = v1.iter(); // becomes item obj
    for el in v1_iter{
        println!("{}", el);
    }
}