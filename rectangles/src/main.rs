fn main() {
    let shape1 = Rectangle { 
        height: 100,
        width: 20,
    };

    let area1 = shape1.get_area();
    println!("rect 1 is {:#?}", shape1);
    println!("area of shape 1 is {}", area1);
}

#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32
}

//Defining rectagle based extension methods
impl Rectangle {
    //Method
    fn get_area(&self) -> i32 {
        self.height * self.width
    }
    //Method with other parameter
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.height >  other.height && self.width > other.width
    }
    //Function 
    fn square(width: i32) -> Rectangle {
        Rectangle {
            height: width,
            width: width,
        }
    }
}
