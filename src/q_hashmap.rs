use std ::collections::HashMap;

fn desird_output (pair : Vec<(String,i32)>)-> HashMap<String, i32>{
    let mut hashmap = HashMap::new ();
    for (key,value) in pair {
        hashmap.insert(key,value);
    }
    return hashmap;
}

fn main(){
    let newvecpair = vec![(String :: from ("Arpita"), 22),(String::from ("Sumit"),18) ];
    let  ans = desird_output(newvecpair);
    println!("The output is this {:?}", ans);
    
}