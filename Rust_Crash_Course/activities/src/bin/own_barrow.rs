#[allow(unused_variables)]
#[allow(dead_code)]
enum Light {
    Bright,
    Dull
}

fn display_light(light: &Light){
    match light{
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

fn main(){
    let dull = Light::Dull;
    // display_light(dull);
    // in the above case the variable is moved
    // and destroyed by the function
    display_light(&dull);
    display_light(&dull);
}