fn main() {
    fib(15);
}

fn fib(n : i32) {
    let mut res = 1;
    let mut prev_res = 0;
    for num in (1..n) {
        res = res + prev_res;
        println!("{} result is {}", num, res);
        prev_res = res;
    }
}

 