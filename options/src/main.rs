fn main() {
    let some_number = Some(5);
    if some_number.is_some(){
        println!("We got a thing");
    } else {
        println!("We don't got a thing");
    }

    let other_number = match some_number {
        Some(i) => i + 5,
        None => -1
    };

    println!("Match result is {}", other_number);

}
