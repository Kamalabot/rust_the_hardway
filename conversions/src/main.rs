#![allow(warnings)]
#![allow(unused_imports)]

use coversions::*;
mod prac_mod0;
use prac_mod0::*;
fn main() {
    // custom_try_from_error();
    // opt_to_res();
    // res_to_opt();
    let _ = auto_conversion();
    as_conv();
    let c1 = Celsius(62.0);
    let f1 = Farenheit { value: 25.0 };
    // debug print will be same as their init process
    println!("Direct inited C and F {:?} {:?}", &c1, &f1);

    let c1_to_f: Farenheit = c1.into();
    let f1_to_c: Celsius = f1.into();

    println!("Converted temperatures {:?} {:?}", c1_to_f, f1_to_c);
    println!(
        "After Display trait implementation\n {} {}",
        c1_to_f, f1_to_c
    );

    let sc1 = "25.0";
    let sf1 = "165.0";
    let sc1_t: Celsius = sc1.try_into().unwrap();
    let sf1_t: Farenheit = sf1.try_into().unwrap();
    println!("Working on Trying C: {}, F: {}", sc1_t, sf1_t);
    // also direct use of try_from
    let sc2_t = Celsius::try_from("36.5").unwrap();
    let sf2_t = Farenheit::try_from("106.5").unwrap();
    println!("Working on Direct try C: {}, F: {}", sc2_t, sf2_t);
    let sc3_c = Celsius::new(65.0);
    println!("Using new: {}", sc3_c);
}
