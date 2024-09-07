#[allow(dead_code)]
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_exp_calc(inte: i32) -> i32 {
    // function is simply
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    inte // returns the inte back
}

fn gen_workout(inte: i32, ran_num: i32) {
    // let exp_closure = |num| -> i32 {
    let mut exp_closure = Cacher::new(|num| {
        // closure implements Fn
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }); // need to understand this more throughly...
    if inte < 25 {
        // println!("Do {} pushups", simulated_exp_calc(inte));
        // println!("Do {} situps", simulated_exp_calc(inte));
        // println!("Do {} pushups", exp_closure(inte));
        println!("Do {} pushups", exp_closure.value(inte)); // when using Cacher Closure
                                                            // println!("Do {} situps", exp_closure(inte));
        println!("Do {} situps", exp_closure.value(inte)); // when using Cacher Closure
    } else {
        if ran_num == 3 {
            // println!("Do {} pushups", simulated_exp_calc(inte));
            // println!("Do {} pushups", exp_closure(inte));
            println!("Do {} Pushups", exp_closure.value(inte)); // when using Cacher Closure
        } else {
            // println!("Today's run for {} minutes", simulated_exp_calc(inte));
            // println!("Today's run for {} minutes", exp_closure(inte));
            println!("Today's run for {} minutes", exp_closure.value(inte));
        }
    }
}

fn main() {
    let sim_int = 7;
    let rn_num = 10;
    gen_workout(sim_int, rn_num);

    let x = 4;
    // very simple closure
    let eql_to_x = |z: i32| z == x;
    let y = 3;
    assert!(eql_to_x(y));
}
// need to learn about Fn, Fnonce, Fnmut
