const MAX_POINTS : u32 = 10000;

fn main() {
    //Mutable variables
    let mut x = 5;
    println!("X is {}", x);
    x = 6;
    println!("X is {}", x);

    //Contants
    println!("Max is {}", MAX_POINTS);

    //Shadowing 
    let y  = 6;
    let y = y * 10;
    println!("y is {}", y);

    let name = "Elliot";
    let name : usize = name.len(); //Don't HAVE to specify the type here but I like to 
    println!("name is {}", name);

    //Tuples
    let eh = (500, "Elliot");
    // reference tuples
    println!("EH num is {} and EH name is {}", eh.0, eh.1);    
    // Destructure tuples 
    let  (ehnum, ehname) = eh;
    println!("EH num is {} and EH name is {}", ehnum, ehname);    

    //Arrays - Stack allocated
    let a = [1,2,3,4,5];
    println!("first element is {}", a[0]);
    
    //Runtime error 
    //let index = 10;
    // Results in a panick and prevents invalid memory access (as opposed to c / c++)
    //println!("tenth element is {}", a[index]);

    //functions
    another_function();
}

