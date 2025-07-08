fn main() {
    let  ans;
    let str1 = String::from("Amit");
    {let str2 = String::from("Arpita");
    ans = longest(&str1,&str2);
        }    println!("{}", ans);


}
fn longest<'a>(a:&'a str, b:&'a str) ->&'a str {
    if a.len() > b.len(){
        return a;
    }
    return b;

}

// /when reference is used the lifespan of str2 end in the line  7 as it is now borrowed ans cant be printed after line 7 
// / To solve this we will specify the span of all the strs