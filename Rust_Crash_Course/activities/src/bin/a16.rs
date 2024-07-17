// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
struct Locker{
    name: String,
    // * The locker assignment should use an Option<i32>
    // enum Option<T>{
    //      Some(val),
    //      None
    // }
    lock: Option<i32> 
}

fn assigned_locker(locker: &Locker){
    println!("Student Name: {:?} \n Locker Number: {:?}",
            locker.name, locker.lock);
}

fn main() {
    let std1 = Locker{
        name: "Kani".to_owned(),
        lock: Some(56),
    };
    let std2 = Locker{
        name: "Null".to_owned(),
        lock: None,
    };
    let std_vec = vec![std1, std2];
    for lk in std_vec{
        // None can be checked for equality
        if lk.lock != None{
            assigned_locker(&lk);
        } 
        else {
            println!("Got only name: {}", lk.name);
        }
    }
}
