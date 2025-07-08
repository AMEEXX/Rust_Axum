
// mod length_string;

// fn main() {
//     println!("{}", (is_even(-11)))
// }
// fn is_even(num : i32) -> bool{
//     if num%2 == 0 {
//         return true;
//     } 
//     return false;
// }use chrono :: {Local};

use chrono :: Local;
fn main() {
    let now = Local ::now();
    println!("{}",now)
}