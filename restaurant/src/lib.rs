mod front_of_house;

#[cfg(test)]
mod tests {
    use crate::eat_at_restaurant2;

    #[test]
    fn it_works() {
        eat_at_restaurant2();
        assert_eq!(2 + 2, 4);
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();

    front_of_house::hosting::add_to_wait_list();

    use self::front_of_house::hosting;
    hosting::add_to_wait_list();
}

fn serve_order() {}

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
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("hoge");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("{:?}, {:?}", order1, order2);
}