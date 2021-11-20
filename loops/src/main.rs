fn main() {
    'label1: loop {
        println!("Hello, world! Again!");
        loop {
            break 'label1;
        };
    };

    let mut num = loop {
        break 3;
    };
    println!("num={}", num);

    while num != 0 {
        println!("num={}", num);
        num -= 1;
    }

    let arr = [1, 2, 3, 4];
    for e in arr {
        println!("e={}", e)
    }

    for e in 1..4 {
        println!("e={}", e);
    }

    for e in (1..4).rev() {
        println!("e={}", e);
    }
}
