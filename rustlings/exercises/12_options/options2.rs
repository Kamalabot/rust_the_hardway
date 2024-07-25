fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            Some(assert_eq!(word, target));
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;  // assigning range of values
        let mut optional_integers: Vec<Option<i8>> = vec![None];
        // initialize the vector with None Option
        for i in 1..=range {
            // pushing the Options into the vectors
            optional_integers.push(Some(i));
        }

        let mut cursor = range; // assign range to cursor variable

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // learnt about the solution from https://www.reddit.com/r/rust/comments/hzth8f/rustlings_options2_question/
        // need to understand what is happening
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
