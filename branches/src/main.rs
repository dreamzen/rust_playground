fn main() {
    let num: i64 = 99;
    if num < 100 {
        println!("Hello, world!");
    } else {
        println!("hi");
    }

    let num2 = if num < 100 {
        3
    } else {
        4
    };

    println!("num2={}", num2);
}
