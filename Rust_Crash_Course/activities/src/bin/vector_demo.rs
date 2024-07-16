#[allow(dead_code)]
// introduces the vectors 
struct Phone{
    make: String, // this string has to be owned
    qty: i32,
}
impl Phone{
    fn print(&self){
        println!("Make: {} and qty is {}", self.make, self.qty);
    }
}
fn main(){
    let vec1 = vec![1, 2, 3, 5];

    for elem in vec1{
        println!("Elem is {:?} ", elem);
    }
    let mut vec2 = Vec::new(); 

    vec2.push(17);
    vec2.push(27);
    vec2.push(37);
    vec2.push(75);
    vec2.push(76);
    println!("Length before pop is: {}", vec2.len());
    vec2.pop();
    println!("Length after pop is: {}", vec2.len());

    println!("Extracting elements : {}", vec2[1]);

    for elem in vec2{
        println!("Elem is {}", elem);
    }
    let mut vec3 = Vec::new(); 
    vec3.push(Phone{make: "mi".to_owned(),qty: 2});
    vec3.push(Phone{make: "one+".to_owned(),qty: 25});
    vec3.push(Phone{make: "samsung".to_owned(),qty: 12});
    // understand how concat occurs

    // using impl in the below code
    for elem in vec3{
        elem.print();
    }

}
