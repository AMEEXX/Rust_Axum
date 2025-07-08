fn main( ){
    let num = vec! [1,2,3,4];
    let num_iter = num.iter();
    let sum : i32 = num_iter.sum();
    println!("{}", sum());
    for value in num_iter.sum {
        println!("{}", value);

    }
    println!("{:?}",num)
} // Consumint iterators give a new variable and the old iterator is gayab no iterator anymore...but adaptive iterator makes a new iterator with a functiont used in it.