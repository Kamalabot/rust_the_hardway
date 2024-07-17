#[allow(dead_code)]
struct Survey{
    q1: Option<i32>,
    q2: Option<i32>,
    q3: Option<i32>,
}

fn main(){
    let s0 = Survey{
        q1: Some(2),
        q2: None,
        q3: None,
    };
    let s2 = Survey{
        q1: Some(5),
        q2: Some(4),
        q3: None
    };
    let s3 = Survey{
        q1: Some(3),
        q2: Some(1),
        q3: None
    };
    let survey_vec = vec![s0, s2, s3];
    let mut idx = 0;
    for s1 in survey_vec{
        println!("This is survey number: {}", idx);
        match s1.q1 {
            Some(val) => println!("{}", val),
            None => println!("No value provided for q1"),
        }
        match s1.q2 {
            Some(val) => println!("{}", val),
            None => println!("No value provided for q2"),
        }
        match s1.q3 {
            Some(val) => println!("{}", val),
            None => println!("No value provided for q3"),
        }
        idx += 1;
    }
}