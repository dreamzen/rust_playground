use deref::MyBox;

fn main() {
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("world"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
