trait Summary {
    fn summarize(&self) -> String {
        return String::from("The trait is");
    }
}

struct User {
    name : String,
    age : i32,
}

impl Summary for User {
    fn summarize(&self) ->String {
        return format!("The age of {}, is {} years",self.name,self.age);
    }
}

fn main() {
    let  user = User {
        name : String::from("Arpita"),
        age: 21,
    };
    println!("{}", user.summarize())
}