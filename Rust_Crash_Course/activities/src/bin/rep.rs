fn main(){
    loop{
        println!("This is made to print once...");
        break
    }
    let mut r1 = 0;
    loop {
        println!("This is printing {} time", r1);
        r1 += 1;
        if r1 > 3{
            break;
        }
    }
    println!("Walkforward loop is done...");
    
    let mut r2 = 3;
    loop {
        println!("This is printing {} time", r2);
        r2 -= 1;
        if r2 == 0{
            break;
        }
    }
    println!("Walkback loop is done...");

    // repetition using while 
    let mut wd1 = 0;
    while wd1 < 4{
        println!("I am still less than 4 {:?}", wd1);
        wd1 += 1;
    }
    println!("There I am outside and bigger...");

}