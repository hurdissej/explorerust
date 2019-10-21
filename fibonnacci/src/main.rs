fn main() {
    let numberOfFibsToPrint = 10;
    let mut start = (0, 1);
    for i in (0..numberOfFibsToPrint) {
        println!("{}", start.0);
        start = (start.1, start.0 + start.1);
    }
}
