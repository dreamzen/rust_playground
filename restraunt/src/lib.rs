mod front_of_house;

fn serve_order() {}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }
}

use back_of_house::Breakfast;

fn eat_breakfast() {
    let breakfast = Breakfast::summer("Rye");
    println!("breakfast is {}", breakfast.toast);
}
