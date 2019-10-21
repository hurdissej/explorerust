use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i32> = vec![4, 3, 2, 10, 324, 21, 8, 8];
    numbers.sort();

    let mut tot = 0;
    let mut contents : HashMap<i32, i32> = HashMap::new();

    for num in &numbers {
        let count = contents.entry(*num).or_insert(0);
        *count += 1;

        println!("{}", num);
        tot += num;
    }

    let mut maxVal = 1;
    let mut mode = 0;
    for (k, v) in &contents {
        if v > &maxVal {
            mode = *k;
            maxVal = *v;
        }
    }
    
    //TODO if there are no repeats write out "NO MODE"
    println!("Mode: {}", mode);

    let avg = tot as f32 / numbers.len() as f32;
    
    let midway = numbers.len() / 2;

    let median: f32;
    if numbers.len() % 2 == 0 {
        median = (numbers[midway] + numbers[midway - 1]) as f32 / 2 as f32;
    } else {
        median = numbers[midway] as f32;
    }
    println!("Mean: {}", avg);
    println!("Median: {}", median);
}
