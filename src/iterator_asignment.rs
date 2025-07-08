fn main() {
    let num = vec![12,3,4,5,7,9,13];
    // let num_iter = num.iter();
    // let num_iter2 = num_iter.filter(|x| *x % 2 == 1);
    // let num_iter3 = num_iter2.map(|x| x*2); can be done i one step

    // let mut ans = Vec:: new(); 
    // for val in num_iter3 {
    //     ans.push(val);

    // } this can be done using collect
    let num_iter = num.iter().filter(|x| *x%2 ==1).map(|x| x*2);

    let ans: Vec<i32> = num_iter.collect();
    println!("{:?}",ans);
} 