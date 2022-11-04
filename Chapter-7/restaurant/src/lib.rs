mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn taken_order() {}
        fn serve_order() {}
        fn take_payment() {}
        pub mod back_of_house {
            fn fix_incorrect_order(){
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
        }
    }

}

pub fn eat_at_restaurant() {
    //Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative Path
    front_of_house::hosting::add_to_waitlist();
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
}
