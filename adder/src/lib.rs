#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "should not work")]
    fn another() {
        panic!("this should not work");
    }

    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 10,
            height: 12,
        };
        let r2 = Rectangle {
            width: 9,
            height: 10,
        };
        assert!(r1.can_hold(&r2));
        assert_eq!(true, r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
        assert_eq!(
            true,
            r2.can_hold(&r1),
            "small can not hold large r1 = {:?}",
            r1
        );
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
