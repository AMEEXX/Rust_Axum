use std::collections::HashMap;
fn main() {
    let mut hashmap = HashMap :: new();
    hashmap.insert(String::from("harkirat"),22);
    hashmap.insert(String::from("arpita"),22);
    let first= hashmap.get("arpita");
    match first {
        Some(value) => println!("The age is : {}",value),
        None => println!("there is no person like that"),
    }

}