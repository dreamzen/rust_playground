fn main() {
    let str = String::from("Hello World");
    let word = first_word(&str);
    println!("word = {}", word);
    println!("word = {}", first_word("hello world"));

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return s;
}