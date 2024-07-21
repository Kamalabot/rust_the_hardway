#[allow(dead_code)]
pub mod funced;

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    // use crate::*;
    use super::*;
    // #[test] 
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_returns_result() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two and two has to be four"))
        }
    }
    #[test] 
    #[ignore] // test is ignored
    fn it_fails() {
        panic!("It panics like a kid...");
    }
    #[test]
    fn test_greet_name() {
        assert!(funced::greet_name("earthlings".to_owned()).contains("earthlings"));
    }
    #[test]
    fn test_greet_name_fail() {
        let result = funced::greet_name("new_lanka".to_owned());
        assert!(!funced::greet_name("earthlings".to_owned()).contains("earthlings"),
                "Greeting does contain the name, so tes is wrong {}", result);
    }
    //#[test]
    fn test_can_hold(){
        let st1 = funced::Rectangle{
            width:5,
            height:5
        };
        let small = funced::Rectangle{
            width:3,
            height:3
        };
        assert!(st1.can_hold(&small));
        assert_eq!(st1.area(), 25);
        assert_eq!(small.area(), 9);
        assert!(!small.can_hold(&st1));
    }
    #[test]
    #[should_panic(expected="Function has to implement logic that panics when guess is > 100")]
    fn test_panicker(){
        funced::func_that_panics(120); // idea is the error get thrown manually
    }

}
