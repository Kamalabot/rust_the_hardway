fn password_validator0(passwd: String) -> i32 {
    if passwd.chars().all(char::is_alphabetic) {
        2
    } else if passwd.len() >= 8
        && passwd
            .chars()
            .any(|c| c.is_ascii_punctuation() || c.is_numeric())
    {
        4
    } else if passwd.len() >= 8 && passwd.chars().any(char::is_numeric) {
        3
    } else if passwd.len() < 8 && passwd.chars().all(char::is_numeric) {
        1
    } else {
        0
    }
}
fn password_validator(passwd: String) -> i32 {
    match passwd {
        _ if passwd.chars().all(char::is_alphabetic) => 2,
        _ if passwd.len() >= 8
            && passwd.chars().any(|c| c.is_ascii_punctuation())
            && passwd.chars().any(char::is_numeric) =>
        {
            4
        }
        _ if passwd.len() >= 8 && passwd.chars().any(char::is_numeric) => 3,
        _ if passwd.len() < 8 && passwd.chars().all(char::is_numeric) => 1,
        _ => 0,
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use crate::password_validator;

    #[test]
    fn test_very_weak() {
        assert_eq!(password_validator("12345".to_owned()), 1);
    }
    #[test]
    fn test_weak() {
        assert_eq!(password_validator("abcdef".to_owned()), 2);
    }
    #[test]
    fn test_strong() {
        assert_eq!(password_validator("abc123xyz".to_owned()), 3);
    }
    #[test]
    fn test_very_strong() {
        assert_eq!(password_validator("1e57#a0r".to_owned()), 4);
    }
}
