fn main() {
    let num : Vec<i32> = vec! [1,2,3,4,5];
    let num_iter = num.iter();
    let num_iter2 = num_iter.map(|x| x +1);
    for value in num_iter {
        println!("{}", value);
    }
}