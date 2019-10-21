fn main() {
    let string_literal = "some string"; //stored on the stack 

    let mut s: String = string_literal.to_string(); //Convert it to a mutable String type

    println!("{}", s);

    // this could also be done with the String::from() method

    s.push_str(" and another string");

    println!("{}", s);

    let hello = String::from("Hello ");
    let world = String::from("world!");
    let hello_world  = hello + &world;
    println!("{}", hello_world);

    //Slicing strings 
    //As the strings are stored as UTF8 encoded bytes it is not clear what a direct index is referring to
    //Hence you need to call an extension method on the string to convert it into something useful e.g.

    let b = hello_world.chars();
    for (i, c) in b.enumerate() {
        if i == 4 {
            println!("{}", c);
        }
    }
}
