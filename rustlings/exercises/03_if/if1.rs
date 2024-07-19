use std::cmp::Ordering;
fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // if a > b{
        // a
    // } else if a < b {
        // b
    // } else {
        // a
    // }
    // using cmp and match
    match a.cmp(&b) {
        Ordering::Equal => a,
        Ordering::Greater => a,
        Ordering::Less => b
    }
    // Do not use:
    // - another function call
    // - additional variables
}

fn main() {
    // You can optionally experiment here.
    let f = format!("{}", bigger(15, 6));
    println!("{}", f); // last line of the fn is fine w/o semi-colon
    println!("Adding another line, breaks...")
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
