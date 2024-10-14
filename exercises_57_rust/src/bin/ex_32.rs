use std::io::stdin;

fn main() {
    println!("Karvonen's Heart Rate");
    let mut rest = String::new();
    let mut age = String::new();
    println!("Provide your resting heart rate: ");
    stdin().read_line(&mut rest).unwrap();
    println!("Provide your age: ");
    stdin().read_line(&mut age).unwrap();
    let nrest = rest.trim().parse::<i32>().unwrap();
    let nage = age.trim().parse::<i32>().unwrap();
    let mut start = 55;

    println!("Resting Rate: {nrest} Age: {nage}");
    println!("intensity    | Heart Rate");
    loop {
        let hrate = karvonen(nage, nrest, start);
        println!("{start}           | {hrate}");
        start += 5;
        if start >= 95 {
            break;
        }
    }
}

fn karvonen(age: i32, rest_rate: i32, intensity: i32) -> i32 {
    let temp1 = (220 - age) - rest_rate;
    let temp2 = temp1 as f32 * intensity as f32 / 100.0;
    (temp2 + rest_rate as f32).abs() as i32
}
