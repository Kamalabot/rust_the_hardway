#[allow(dead_code)]
// ..user2 option caught my attention
// the structs are mutable too
#[derive(Debug)]
struct User {
    username: String,
    mail: String, 
    active: bool,
    signons: u64
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32{
        self.width * self.height
    }
    fn can_hold(&self, other: Rectangle) -> bool {
        other.height < self.height && other.width < self.width
    }
}
struct QuitMessage; // unit struct
struct WriteMessage(String); // tuple struct
struct ChangecolorMessage(i32, i32, i32, i32); // tuple struct
fn main(){
    let mut user1 = User{
        username:"Chakana".to_owned(),
        mail: "adm.in".to_owned(),
        active: true,
        signons: 2
    };
    user1.signons += 3;
    println!("{:#?}", user1); 

    let user2 = make_user("gotout".to_owned(),
                    "new.may.com".to_owned());
    
    let user3 = User {
        username: "thrird".to_owned(),
        mail:"three.com".to_owned(),
        ..user2
    };
    println!("{:?}", user3);

    let rect1 = Rectangle{
        width:5,
        height:5
    };

    println!("The area of {:?} is {}", rect1, rect1.area());
    
    let rect2 = Rectangle{
        height:3,
        width:3,
    };
    println!("{:?}", rect1.can_hold(rect2));

}

fn make_user(username: String, mail: String) -> User{
    User{
        username,
        mail,
        active:true,
        signons:1
    }
}
