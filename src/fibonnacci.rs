 fn  main ( ) {
    println_fibonacci(6);

 }
 fn println_fibonacci(num : i32) {
    let mut a = 0;
    let mut b = 1;
    for i in 0..=num {
    if i == 0 {
        println!("{}", a);
    }
    if i ==1 {
        println!("{}", b);

    }
    else {
        let c = a + b;
        println!("{}",c);
        a= b ;b = c;
    }
 }
}