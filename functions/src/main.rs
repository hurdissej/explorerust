fn main() {
    another_function();
    parameterised_function(3, "Y".to_string());

    //Statement 
    let x = 5;
    println!("The value of x is {}", x);
    //Expression 
    let y  = {
        let x = 3;
        x + 1 // <--- expressions don't have semicolons (I wonder how key this will be)
    };

    println!("The value of y is {}", y);

    println!("The value of five is {}", five());

    println!("The value of five plus one is {}", add_one(five()));
}

fn five() -> i32{
    5
}

fn add_one(input : i32) -> i32 {
    input + 1
}

fn another_function(){
    println!("Another function");
}

fn parameterised_function(x : i32, y : String){
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
}