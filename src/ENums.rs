// When something can only of particular type restriction is there like north south etc.
enum Direction {
    North, 
    East,
    South,
    West,
}
enum Shape {
    Rectangle(f64,f64),

    Square(f64),
}

fn main () {
    let myrect = Shape :: Rectangle(1,2);
    print(Direction::North);
}
fn print(direction : Direction) { // see how we are passing the argument as name smths then the struct with a semicolon
    println!("hi there");
}

fn calculate_are(shape : Shape) {
    //pattern matching
    match shape{
        Shape :: Rectangle(a,b) => a *  b,
        Shape :: Square(a) => 3.13 * a *a,

    } 
}