fn main() {
    let u = User {
        active: true,
        username: String::from("Tom"),
        sign_in_count: 2,
        email: String::from("hello@abc.com"),
    };
    println!("active={}", u.active);

    let u2 = build_user(String::from("Jack"), String::from("hello@cde.com"));
    println!("name={}", u2.username);

    let u3 = User {
        username: String::from("David"),
        ..u2 // u2's String will MOVE to u3
    };
    println!("u3.email={}", u3.email);
    //using u2.email is not allowed
    // println!("u2.email={}", u2.email);

    let black = color(0, 0, 0);
    println!("{}", black.0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple struct
struct color(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}