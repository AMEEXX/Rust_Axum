fn main(){
    let num = String ::from ("amit");
    let num2 = String ::from ("arpita");
    let ans = comp(num,num2);
    println!("{}", ans);
}
fn comp <T : std::cmp ::PartialOrd>(a:T , b: T)-> T{
    if a > b {
        return a;
    }
    return b;

}