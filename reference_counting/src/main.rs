use reference_counting::node;
use reference_counting::{ConsoleSender, LimitTracker};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

enum List {
    Cons(String, Rc<List>),
    Nil,
}

fn main() {
    let t = Cons("3".to_string(), Rc::new(Nil));
    let a = Rc::new(t);
    println!("reference count of a = {}", Rc::strong_count(&a));
    let b = Cons("1".to_string(), Rc::clone(&a));
    println!("reference count of a = {}", Rc::strong_count(&a));
    if let Cons(m, mut n) = b {
        println!("value in b = {}", m);
        n = Rc::new(Nil); // remove reference to a
    }
    println!("reference count of a = {}", Rc::strong_count(&a));
    {
        let c = Cons("2".to_string(), Rc::clone(&a));
        println!("reference count of a = {}", Rc::strong_count(&a));
    }
    println!("reference count of a = {}", Rc::strong_count(&a));

    let sender = ConsoleSender;
    let mut limit_tracker = LimitTracker::new(&sender, 10);
    limit_tracker.set_value(11);

    let v = List2::Cons2(1, RefCell::new(Rc::new(List2::Nil2)));
    let t = v.tail().expect("");
    println!("{:?}", *t.borrow());
    println!("{:?}", v);

    let leaf = Rc::new(node::Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(node::Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, RefCell<Rc<List2>>),
    Nil2,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            List2::Cons2(_, item) => Some(item),
            List2::Nil2 => None,
        }
    }
}
