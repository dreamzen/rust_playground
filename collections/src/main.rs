use std::collections::HashMap;

fn main() {
    // let mut v = Vec::new();
    let mut v = vec![1, 2, 3];
    let ele1 = &v[0];
    v.push(4);
    // immutable reference and mutable reference in the same scope is not allowed
    // println!("ele1 = {}", ele1);

    for i in &v {
        println!("{}", i);
    }

    let s = "Hello World!".to_string();
    println!("s = {}", s);
    let s2 = s + " my";
    println!("s2 = {}", s2);
    let s3 = String::from(" friend!");
    let s4 = s2 + &s3;
    println!("s4 = {}", s4);

    let str = "大家好严helloते";
    println!("len = {}", str.len());
    let chars = str.chars();
    for c in chars {
        println!("{}", c);
    }

    for b in str.bytes() {
        println!("{}", b);
    }

    let mut hash_map: HashMap<String, i32> = HashMap::new();
    let key = "hello".to_string();
    hash_map.insert(key, 3);

    let key2 = "hello".to_string();
    // must use &, key is moved while inserting, so cannot use key here
    let v = hash_map.get(&key2);
    println!("v={}", v.expect("error"));

    hash_map.insert(String::from("hi"), 5);

    for (k, v) in &hash_map {
        println!("{}:{}", k, v);
    }

    hash_map.entry(String::from("hello")).or_insert(5);
    hash_map.entry(String::from("hey")).or_insert(10);

    println!("{:?}", hash_map);

    let v = hash_map.entry(String::from("hey")).or_insert(11);
    *v = 100;
    println!("{:?}", v);
}
