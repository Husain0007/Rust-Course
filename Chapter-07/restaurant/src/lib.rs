mod front_of_house;

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute Path
    hosting::add_to_waitlist();
    //Relative Path
    front_of_house::hosting::add_to_waitlist();
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
}
