#![allow(dead_code, unused)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist!");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
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
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;
// 相对路径
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用use引入作用域
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 公有字段可以访问和修改
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 报错：私有字段无法修改
    // meal.seasonal_fruit = String::from("blueberries");

    // 报错：具有私有字段的结构体无法创建
    // let meal = back_of_house::Breakfast {
    //     toast: String::from("124"),
    //     seasonal_fruit: String::from("12"),
    // };

    // 公有枚举的所有类型都是公有的
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}