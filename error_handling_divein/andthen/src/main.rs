// and_then is used with both
// option and results

fn main() {
    // println!("Hello, world!");
    let num = Some(4);
    let result = num.and_then(sqr_if_even);
    println!("{:?}", result);

    let num = Some(3);
    let result = num.and_then(sqr_if_even);
    println!("{:?}", result);

    let res = parse_num("5").and_then(square_if_positive);
    let rep = parse_num("-5").and_then(square_if_positive);

    println!("Res is {:?}", res);
    println!("Rep is {:?}", rep);
}

fn sqr_if_even(x: i32) -> Option<i32> {
    if x % 2 == 0 {
        Some(x * x)
    } else {
        None
    }
}

fn parse_num(s: &str) -> Result<i32, &str> {
    s.parse::<i32>().map_err(|_| "parse error")
}

fn square_if_positive(x: i32) -> Result<i32, &'static str> {
    if x > 0 {
        Ok(x * x)
    } else {
        Err("Number is not positive")
    }
}
