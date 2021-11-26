fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        4 => println!("four"),
        // 5..10 (exclusive range) is not supported
        5..=10 => println!("five through ten"),
        _ => println!("anything"),
    }

    let c = 'a';
    match c {
        // 'a'..'x' (exclusive range) is not supported
        'a'..='x' => println!("a through x"),
        'd'..='z' => println!("d through z"),
        _ => println!("anything"),
    }

    // named variables
    let x = Some(10);
    let y = 5;
    match x {
        Some(50) => println!("Got 50 by match"),
        Some(y) => println!("Got y = {} by match", y), // y is shadowed
        _ => println!("Got other value by match"),
    };
    println!("Finally x = {:?}, y = {}", x, y);

    // destructure struct
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    let Point { x: a, y: b } = p;
    assert_eq!(1, a);
    assert_eq!(2, b);
    // a shorter way
    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    match p {
        Point { x, y: 2 } => println!("hit y = 2, where x = {}", x),
        Point { x: 0, y } => println!("hit x = 0, where y = {}", y),
        Point { x, y } => println!("other cases, x = {}, y = {}", x, y),
    }

    // destructure enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // destructure nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // mixture of structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignore using '_'
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // using single '_' will NOT move the data
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // s will not be moved! but is we use _x, s will be moved!
        println!("found a string");
    }
    println!("{:?}", s);

    // ignore using '..'
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, ..) => {
            println!("First number: {}", first)
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("First number: {}, last number: {}", first, last)
        }
    }

    // match guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // @
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7, // use @ to save the variable to `id_variable`
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            // `id` here is not a variable, so we can't use it
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
