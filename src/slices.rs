fn main() {
    let string = String::from("Hi arpita ");
    let stringptr = & string; // this is a refernce ptr for the view of the complte string 
    let mut index = 0;

    for i in string.chars() {
        if i == ' ' {
            break;
        }
        else {
            index += 1;
        }
    }let strr = &string[0..index];
    println!("{}", strr);
}

 
// if i write let string = "Hello Arpita" this is a string literatl directl stored in the binary file.
