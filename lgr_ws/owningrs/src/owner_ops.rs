/// Function takes ownership of the incoming args
pub(crate) fn takes_ownership(in_val: String){
    println!("The in_val is {}", in_val);
    // in_val is lost here
}

/// Integer is copied
pub(crate) fn makes_copy(int_in: i32){
    println!("Printing Integers: {}", int_in);
}

/// Transfers Ownership
pub(crate) fn transfer_ownership() -> String{
    let t_str = String::from("Testing");
    t_str
}

/// Takes and givesback Ownership
pub(crate) fn takes_and_gives(recv_string: String) -> String{
    recv_string
}

pub(crate) fn calc_len_wo_barrow(in_str: String) -> (String, usize){
    let n = in_str.len();
    (in_str, n)
}

pub(crate) fn calc_len_w_barrow(in_str: &str) -> usize {
    in_str.len()
}
/// show how &mut is required for changing the string'
pub(crate) fn change_str(in_str: &mut String){
    in_str.push_str(" Get In...")
}

// Below function will raise error, even if not called
//fn dangle(in_str: &mut String) -> &String{
    // will inform cannot return ref to local vars
    //let s_d = String::new();
    //&s_d // this reference is invalid
//}

pub(crate) fn first_word(in_str: &String) -> usize {
    let bytes = in_str.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    in_str.len()
}

pub(crate) fn first_word_slc(in_str: &String) -> &str{
    let bytes = in_str.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &in_str[..i];
        }
    }
    &in_str
}