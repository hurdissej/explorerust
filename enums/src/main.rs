fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
}

enum IpAddrKind {
    V4(String),
    V6(String),
}