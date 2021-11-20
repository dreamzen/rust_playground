fn main() {
    let mut s = String::from("Hello");
    s.push_str(" world!");
    println!("{}", s);

    let s2 = s; // move
    println!("{}", s2);
    // println!("{}", s);

    let s3 = s2.clone();
    println!("{} {}", s2, s3);

    takes_ownership(s3);
    // s3 is invalid
    // println!("{}", s3);

    let v1 = 5;
    let v2 = v1;
    println!("{} {}", v1, v2);

    let str1 = "hello world!";
    let str2 = str1;
    println!("{} {}", str1, str2);
}

fn takes_ownership(str: String) {
    println!("take ownership of {}", str);
}