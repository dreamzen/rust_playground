fn main() {
    let addr1 = IpAddr::V4(String::from("192.0.0.1"));
    let addr2 = IpAddr::V6(String::from("::1"));

    println!("addr1={:?}", addr1);
    println!("addr2={:?}", addr2);

    let has_something = Some(5);
    println!("some = {}", has_something.expect("error"));
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}