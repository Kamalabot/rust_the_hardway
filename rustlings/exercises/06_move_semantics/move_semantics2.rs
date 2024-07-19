fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // signature informs mutable barrowing
    let mut vec = vec.to_vec();
    // assigned local variable has to be made mutable

    vec.push(88); // processing

    vec // returned after process
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&mut vec0); // this is mutable barrowing

        assert_eq!(vec0, [22, 44, 66]);
        // below test is not there in move_sematics1
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
