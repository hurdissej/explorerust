fn main() {
    let mut input = String::from("first");

    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut result = String::new();
    for (num, letter) in input.chars().enumerate()  {
        if(num == 0 && vowels.contains(&letter)){
            println!("First letter");
            result.push_str(&input);
            result.push_str("-hay");
        } else if !vowels.contains(&letter) {
            println!("Consonant");
            result.push_str(&input[num+1..]);
            result.push_str("-");
            result.push(letter);
            result.push_str("ay");
            break;
        }
    }

    println!("{}", result);
}
