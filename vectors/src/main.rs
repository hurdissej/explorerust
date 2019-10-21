fn main() {
    let mut v : Vec<i32> = Vec::new();
    v.push(0);
    v.push(6);
    v.push(0);
    v.push(4);
    v.push(9);
    v.push(1);

    let third: &i32 = &v[2]; //If this referenced an element that does not exist it will cause a "Panic"
    let orThird: Option<&i32> = v.get(2);

    let does_not_exist  = v.get(100); //This produces a None option which is more user friendly than crashing the program

    for i in &v {
        println!("{}", i);
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Float(6.5),
        SpreadSheetCell::Text(String::from("Blue"))
    ];

    for a in &row {
        match a {
            SpreadSheetCell::Text(value) => println!("{:?}", value),
            SpreadSheetCell::Float(value) => println!("{}", value),
            SpreadSheetCell::Int(value) => println!("{}", value),
        };
    }
}

