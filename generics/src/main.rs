use std::cmp::PartialOrd;

fn main() {
    println!("Hello, world!");
    let mut arr = Vec::new();
    arr.push(3);
    arr.push(1);
    arr.push(2);
    println!("max is {}", largest(&arr));

    let article = Article {
        title: "Title".to_string(),
        content: "Content".to_string(),
    };
    let tweet = Tweet {
        username: "Username".to_string(),
        content: "Content".to_string(),
    };
    println!("{}", article.get_summary());
    println!("{}", tweet.get_summary());

    println!("{}", get_summary(&article));
    println!("{}", get_summary(&tweet));

    println!("{}", get_summary_2(&article, &article));

    println!("{}", get_summary_3(&article));

    println!("{}", get_summary_4("hello", "hi", &article));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    if list.len() <= 0 {
        panic!("invalid array");
    }
    let mut max = list[0];
    for &l in list {
        if l > max {
            max = l
        }
    }
    return max;
}

pub trait Summary {
    fn get_summary(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn get_summary(&self) -> String {
        format!("title-{}, content-{}", self.title, self.content)
    }
}

impl Summary for Tweet {
    fn get_summary(&self) -> String {
        format!("content-{} by {}", self.content, self.username)
    }
}

fn get_summary(item: &impl Summary) -> String {
    item.get_summary()
}

fn get_summary_2<T: Summary>(item1: &T, item2: &T) -> String {
    format!("{} + {}", item1.get_summary(), item2.get_summary())
}

fn get_summary_3<T>(item: &T) -> String
    where T: Summary {
    item.get_summary()
}

fn get_summary_4<'a, T>(str1: &'a str, str2: &'a str, t: &T) -> &'a str
    where T: Summary {
    println!("summary = {}", t.get_summary());
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}