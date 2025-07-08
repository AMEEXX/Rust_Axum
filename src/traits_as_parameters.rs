trait Summary {
    fn summarize(&self) -> String {
        return String::from("Hi there");
    }
}

struct User {
    name : String,
    age : i32,
}
impl Summary for User {
    fn summarize(&self)-> String{
        return format!("The name of user is {} and the age is {}", self.name, self.age);
    }
}
fn notify(u: impl Summary){ // whatever i takes as input can be anthing that imp the summary
    println!("{}",u.summarize());

}

fn main() {
    let user = User {
        name : String::from("Arpita"),
        age : 34,
    };
    println!("{}", user.summarize());
    notify(user)

}// ifyou ever want to create that takes smthg as input that imp this particular summary then this is how we can do that.