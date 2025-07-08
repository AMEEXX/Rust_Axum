fn main() {
    let index = String :: from("Arpita");
    let ans = print_first_a(index);
    match ans {
        CustomOpt :: Some(value) =>println!("index of a : {}",value),
        CustomOpt :: None => println!("an not found")

    }

}

enum CustomOpt {
    Some(i32),
    None,
}

fn print_first_a (s : String)-> CustomOpt{
    for (index,chr) in s.chars().enumerate() {
        if chr == 'a'  {
            return CustomOpt :: Some(index as i32);
        }
    }
    return CustomOpt :: None;

}