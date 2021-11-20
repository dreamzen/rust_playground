fn main() {
    println!("Hello, world!");

    another_function(5, 'A');

    println!("{}", five());
}

fn another_function(x: i32, c: char) {
    println!("Another function. {} {}", x, c);
}

fn five() -> i32 {
    5
}