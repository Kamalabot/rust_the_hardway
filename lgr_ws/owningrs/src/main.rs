mod owner_ops;
fn main() {
    /*
    Ownership Rules:
    - Each value in rust has a variable that is called its owner
    - There can be only one owner at a time
    - When owner is out of scope, the value and its memory is dropped
     */
    { // s is invalid here as it is yet to be declared
        let _s = String::new();
        // work with s
    }// s is again invalid, as the scope ends
    // following shows ownership in action
    let s = "Henry".to_owned();
    // following shows ownership in action
    // let s1 = s; // will be a move 
    let s1 = s.clone();
    println!("Printing s: {}", s); // will error out if not cloned
    println!("Printing s1: {}", s1); // will print Henry 
    owner_ops::takes_ownership(s1.clone());
    // println!("Trying to print s1: {}", s1);
    let local_int = 6;
    owner_ops::makes_copy(local_int);
    // integers are not required to be barrowed, but can be 
    // makes_copy(&local_int);
    println!("Local int after makes_copy: {}", local_int);

    let got_ownership = owner_ops::transfer_ownership();
    println!("Recieved string: {}", got_ownership);

    let given_back = owner_ops::takes_and_gives(s1);
    println!("Given back: {}", given_back);

    let to_calc = String::from("to_calc");

    let (got_str, size) = owner_ops::calc_len_wo_barrow(to_calc);

    println!("Got String: {} and Size : {}", got_str, size);

    // let new_size = calc_len_w_barrow(&to_calc);
    // above will error, as to_calc is already moved to got_str
    let new_size = owner_ops::calc_len_w_barrow(&got_str);

    println!("Barrow: {} and length: {}", got_str, new_size);

    let mut chng_str = String::from("You there... ");
    owner_ops::change_str(&mut chng_str);

    println!("Mutated Got_str: {}", chng_str);
    /*
    References Rules:
    - At any time there can be one "mutable" reference
    or any number of immutable references
    - References have to be valid always
     */
    let full_string = String::from("New World");
    let half_1 = &full_string[0..3];
    // let half_1 = &full_string[..3];
    let half_3 = &full_string[4..9];
    // let half_3 = &full_string[4..];
    println!("First half: {} and Second Half: {}", half_1, half_3);

    let len_1 = owner_ops::first_word(&full_string);
    // string slice &str is different from String
    println!("Extracted len_1 is {}", len_1);

    let slc_rcv = owner_ops::first_word_slc(&full_string);
    println!("Sliced string with space: {}", slc_rcv);
}
