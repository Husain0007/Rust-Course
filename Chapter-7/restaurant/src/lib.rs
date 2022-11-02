mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn taken_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

}

pub fn eat_at_restaurant() {
    //Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    //Relative Path
    front_of_house::hosting::add_to_waitlist();
}
