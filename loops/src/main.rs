fn main() {
    let mut number = 3;
    while number != 0 {

        println!("Whiling {}", number);
        number = number - 1;
    }
    println!("LIFTOFF!");

    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("Arr value is {}", arr[index]);
        index = index + 1;
    } 

    for element in arr.iter() { 
        println!("Arr value when iter'ing is {}", element);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }

}
