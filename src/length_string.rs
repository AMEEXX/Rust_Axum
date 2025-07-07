

fn my_string_function(s : &str) -> usize{
    s.chars().count() //dont add semicolon here because this is not a statement but a return so as no return word is there dont add ;
}

fn main() {
    let my_string = String :: from("Hello, World");
    let length = my_string_function(&my_string);
    println!("The length of the stringt is {}", length);
}