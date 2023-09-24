struct User {
    //attributes
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle { // impl block stores the functions and methods associated with Rectangle Struct
    fn area(&self) -> u32 {
        self.width * self.height 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("syedhusainmustafa@gmail.com"),
        username: String::from("Husain0007"),
        active: true,
        sign_in_count: 12
    };
    let name = user1.username;
    user1.username =  String::from("Mustafa");

    let user2 = build_user(String::from("kyle@mail.com"),
String::from("kyle123"));

    let user3 = User{
        email: String::from("jim@mail.com"),
        username: String::from("jim123"),
        ..user2 // to fill remaining param using corresponding values from "user2"
    };

    /* Tuple Structs */
    struct Color(i8, i8, i8);
    struct Point(i32, i32, i32);

    /*Use Rectangle Struct with Implementation */

    let rect = Rectangle{
        width: 30,
        height: 50
    };
    let rect1 = Rectangle{
        width: 25,
        height: 40
    };
    let rect2 = Rectangle{
        width: 55,
        height: 40
    };
    let rect3 = Rectangle::square(25);
    println!("the area of the rectangle is {} square pixels.", rect.area());
    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

}

fn build_user(email: String, username: String) -> User {
    User{
        email, // "Field init short hand syntax" i.e.; do not need to specify input parameter when field and input param have the same name.
        username,
        active: true,
        sign_in_count: 1, 
    }
}
