pub mod node;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    max: i32,
    value: i32,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: i32) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Error: over the max!");
        } else if percentage >= 0.9 {
            self.messenger.send("Warning: almost reach the max!")
        } else {
            self.messenger.send("Info: ok!")
        }
    }
}

pub struct ConsoleSender;

impl Messenger for ConsoleSender {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockSender {
        pub msg: RefCell<String>,
    }

    impl MockSender {
        fn new() -> MockSender {
            MockSender {
                msg: RefCell::new("".to_string()),
            }
        }
    }

    impl Messenger for MockSender {
        fn send(&self, msg: &str) {
            *self.msg.borrow_mut() = msg.to_string();
        }
    }

    #[test]
    fn test() {
        let mock_sender = MockSender::new();
        let mut limit_tracker = LimitTracker::new(&mock_sender, 10);
        limit_tracker.set_value(11);
        assert_eq!("Error: over the max!", *mock_sender.msg.borrow());
        limit_tracker.set_value(9);
        assert_eq!("Warning: almost reach the max!", *mock_sender.msg.borrow());
        limit_tracker.set_value(8);
        assert_eq!("Info: ok!", *mock_sender.msg.borrow());
    }
}
