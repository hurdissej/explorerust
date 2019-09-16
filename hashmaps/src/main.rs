use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("TeamA"), 2);
    map.insert(String::from("TeamB"), 3);

    let lookup = String::from("TeamA");
    let a = map.get(&lookup);

    let res = match a {
        Some(i) => *i, 
        None => -1
    };
    println!("{}", res);   
}
