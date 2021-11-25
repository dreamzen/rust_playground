use blog::Post;

fn main() {
    let mut p = Post::new();
    p.add_text("This is my first post!");
    assert_eq!("", p.content());
    p.approve(); // should take no effect
    assert_eq!("", p.content());
    p.request_review();
    assert_eq!("", p.content());
    p.add_text("Add more after review"); // should take no effect
    assert_eq!("", p.content());
    p.approve();
    assert_eq!("", p.content()); // need 2 approves
    p.add_text("Add more after first approve"); // should take no effect
    assert_eq!("", p.content());
    p.approve();
    p.add_text("Add more after final approve"); // should take no effect
    assert_eq!("This is my first post!", p.content());

    //add reject state
    let mut p = Post::new();
    p.add_text("This is my first post!");
    assert_eq!("", p.content());
    p.request_review();
    assert_eq!("", p.content());
    p.reject();
    assert_eq!("", p.content());
    p.approve(); // should take no effect
    p.approve(); // should take no effect
    assert_eq!("", p.content());
}
