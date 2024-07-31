#[allow(dead_code)]
fn main() {
    /* code can be executed here */
}

fn pass_validator(your_pass: String) -> String{
    if your_pass == "nice_pass"{
        "Accepted".to_owned()
    } else {
        "I don't Know you".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_know(){
        assert_eq!(pass_validator("nice_pass".to_owned()), "Accepted".to_owned());
        assert_ne!(pass_validator("nice_ps".to_owned()), "Don't know".to_owned());
    }
}