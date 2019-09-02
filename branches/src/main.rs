fn main() {
    let number = 7; 

    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    fizz_bang(15);
    fizz_bang(4);
    fizz_bang(3);
    fizz_bang(50);

    let expression = false;
    let z = if expression {
        5
    } else {
        6 //Have to be matching types 
    };

    println!("z is equal to {}", z);
}

fn fizz_bang(input : i32) {
    if input % 5 == 0 && input % 3 == 0 {
        println!("FizzBang");
    } else if input % 5 == 0 {
        println!("Bang");
    } else if input % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{}", input);
    }
}