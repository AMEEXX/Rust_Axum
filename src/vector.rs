 fn main() {
    let mut vec : Vec<i32> = vec![1,2,3,4,5,6];
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("The value of vector is {:?}",even_value(&vec) ); 
    println!("The value of vector is {:?}",vec ); 
}
fn even_value(vec2 : &Vec<i32>)-> Vec<i32>{
    let mut newvec = Vec :: new();
    for i in 0..vec2.len(){
        if i%2 ==0 {
            newvec.push(vec2[i]); // yoiu dont need to dereferenc cause the vec2 is the ownder if i used i as val in for loop then i had to dereference the val with *val
        }

    }
    return newvec;

}