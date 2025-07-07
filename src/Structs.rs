/*
struct User {
    active : bool,
    username : String,
    email : String,
    age : i32,
}


fn main() {
    let user = User {
        active : true,
        username : String :: from ("Amit Hota"),
        email : String ::from ("amithota.12@gmail.com"),
        age : 21,

    };
    println!("{}", user.active);
    println!("{}", user.username);
}*/
//
mod rect; 

struct Rect {
    width : i32,
    height : i32,
}
impl Rect { // write the name in upper camelcase
    fn area(&self) -> i32 {
        self.width * self.height 
    }
    fn debug() -> i32 {
        return 1;
    }
}
fn main() {
    let rect1 = Rect {
        width : 34,
        height : 35,
    };
    println!("{}", rect1.area());
    println!("{}", Rect::debug())  // I cannot call the debug from the object but I can directly call it from the class same as a static fucntion in javascript.
}