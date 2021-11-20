fn main() {
    let day = tell_the_day(Day::Sunday);
    println!("today is {}", day);
    println!("today is {}", tell_the_day(Day::Monday));
    println!("today is {}", tell_the_day(Day::Tuesday(24)));
    println!("today is {}", tell_the_day(Day::Wednesday));

    let five = Some(5);
    let six = plus_one(five);
    println!("{}", six.expect("error"));

    let n = plus_one(None);
    println!("{}", n.is_none());

    tell_the_day_2(Day::Tuesday(23));
    tell_the_day_2(Day::Monday);
}

fn tell_the_day_2(day: Day) {
    if let Day::Tuesday(h) = day {
        println!("num in Tuesday = {}", h);
    } else {
        println!("{:?} is not Tuesday", day);
    }
}

fn tell_the_day(day: Day) -> String {
    match day {
        Day::Sunday => String::from("Sunday"),
        Day::Monday => String::from("Monday"),
        Day::Tuesday(h) => {
            println!("num in Tuesday = {}", h);
            String::from("Tuesday")
        }
        _ => String::from("Another day!"),
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
    }
}

#[derive(Debug)]
enum Day {
    Sunday,
    Monday,
    Tuesday(i32),
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}