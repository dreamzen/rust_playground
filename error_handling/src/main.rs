use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("error");
    let f = File::open("text.txt");
    match f {
        Ok(f) => println!("open succeeded"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("file not found"),
            _ => println!("other error"),
        }
    }

    // let f = File::open("hello.txt").expect("failed to open hello.txt");

    let user_name = match read_user_name_from_file() {
        Ok(s) => s,
        Err(e) => "default_user_name".to_string(),
    };

    println!("user_name = {}", user_name);

    let user_name = match read_user_name_from_file_2() {
        Ok(s) => s,
        Err(e) => "default_user_name_2".to_string(),
    };

    println!("user_name_2 = {}", user_name);

    let user_name = match read_user_name_from_file_3() {
        Ok(s) => s,
        Err(e) => "default_user_name_3".to_string(),
    };

    println!("user_name_3 = {}", user_name);
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let f = File::open("user_name.txt");
    let mut f = match f {
        Ok(file) => {
            println!("open succeeded");
            file
        }
        Err(e) => {
            println!("open failed");
            return Err(e);
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(size) => {
            println!("size of name = {}", size);
            Ok(s)
        }
        Err(e) => Err(e),
    }
}

fn read_user_name_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("user_name_2.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_user_name_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("user_name_3.txt")
}