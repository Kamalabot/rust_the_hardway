fn passwordValidator(passwd: String) -> i32 {
    let mut strength = 0;
    // 0 is equal to no password
    // check if it parses to number
    if let Ok(your_pass) = passwd.parse::<usize>() {
        if passwd.len() < 8 {}
    }

    4
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::passwordValidator;

    #[test]
    fn test_very_weak() {
        assert_eq!(passwordValidator("12345".to_owned()), 1);
    }
    #[test]
    fn test_weak() {
        assert_eq!(passwordValidator("abcdef".to_owned()), 2);
    }
    #[test]
    fn test_strong() {
        assert_eq!(passwordValidator("abc123xyz".to_owned()), 3);
    }
    #[test]
    fn test_very_strong() {
        assert_eq!(passwordValidator("1e57#a0r".to_owned()), 4);
    }
}
