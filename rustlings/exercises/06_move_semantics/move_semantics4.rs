fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        // note the source is mutable
        let y = &mut x;  // barrow mutably
        y.push(42); // operate
        let z = &mut x; // barrow again
        z.push(13); // operate
        assert_eq!(x, [42, 13]);
    }
}
