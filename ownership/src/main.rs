fn main() {
    simple_allocation(); // Stack allocations
    string(); // Heap allocation but ownership remains in string() method

    let y = string2(); // Ownership returned to y 
    println!("Y {}", y);

    //Send the immutable reference of y into borrow 
    borrow(&y);

    //Create a mutable string 
    let mut z = string2();

    //And pass a mutable reference so the underlying method can change the data but the ownership remains in z
    borrow_mutable(&mut z);
    println!("appended string is {}", z);

    //Take a slice of hello world
    //These are just normal references 
    let hello = &y[0..5];
    let world = &y[7..12];

    println!("hello is {}", hello);
    println!("world is {}", world);

    let first_word = first_word(&y);
    println!("The first word is {}", first_word);
} // s from string 2 however, is dropped here

fn borrow(s: &String){
    println!("Borrowing the string {}", s);
}

fn borrow_mutable(s : &mut String){
    s.push_str(" some extra test");
}

fn string() { 
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
} 

// Creates a string and then passes the ownership of the string back to the caller
fn string2() -> String {
    let mut s = String::from("hello");
    s.push_str(", world");
    s
}

fn simple_allocation() { 
    // s comes into scope here
    let s = "hello";
    
    //This will result in an error - This is because it is an immutable variable
    //Allocated on the stack -- string() shows how heap allocated variables are used
    // s.push_str(", world"); 

    println!("{}", s);
} // s goes out of scope here and hence it is "dropped"

fn first_word(s : &str) -> &str { //&str is a slice????!
    let bytes = s.as_bytes();

    // iterate through
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i]; // use return to break early and return a reference 
        }
    }

    // If no spaces are found the first word is everything
    &s[..]
}
