# Chapter 5
* Structs are used to group related data.
* To make changes to any attribute of a struct after instantiation the entire instance struct needs to be made mutable.
    ```rust
        struct User{
            username: String,
            email: String,
            age: u32,
            active: bool,
        }
        fn main(){
            let mut husain: User = User { 
                // Making Instance "husain" of Struct "User" mutable
                email: String::from("syedhusainmustafa@gmail.com"),
                username: String::from("Husian007"),
                active: true,
                age: 25
            };
            let age: u32 = husain.age;
            husain.username = String::from("Husain0008");
        }
    ```
* Can create Structs without named fields called **Tuple Structs**.
    ```rust 
        struct Color(i8, i8, i8);
        struct Point(i32, i32, i32);
    ```
* Tuple Structs are usually used in place of a group of variables to signify relation between those variables that may not be obvious from the code.
    * Implementation using unrelated variables.
        ```rust
            fn main(){
                let width: i32 = 30;
                let height: i32 = 50;

                println!("Area of Rectangle is {} square pixels.", area(width, height));
            }
            fn area(width: u32, height: u32) -> u32 {
                width * height
            }
        ```
    * Implementation using a Tuple Struct
        ```rust
        fn main(){
            let rectangle: (u32, u32) = (30, 50);

            println!("Area of Rectangle is {} square pixels.", area(rectangle));
        }
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
        ```
    * Implementation using a Standard Struct
        ```rust
        struct Rectangle {
            width: u32,
            height: u32
        }
        fn main(){
            let rect: Rectangle = Rectangle{
                width: 30,
                height: 50
            };

            println!("Area of Rectangle is {} square pixels.", area(&rect));
        }
        fn area(rectangle: &Rectangle) -> u32 { 
            // Passing in reference to Struct to avoid taking ownership
            rectangle.width * rectange.height
        }
        ```
    * Methods and associated functions for a Struct are written inside the **impl** block. Methods are associated with a specific instance of a struct, associated functions are not. There can be one or more **impl** blocks.
        ```rust 
        struct Rectangle {
            width: u32,
            height: u32
            }
        impl Rectange {
            fn area(&self) ->u32 { // methods are identified by "&self"
                self.width * self.height
            }
            fn can_hold(&self, other:&Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
        impl Rectangle {
            fn square(size: u32) -> Rectangle { 
                // Associated Functions do **not** use "&self"
                Rectangle {
                    width: size,
                    height: size
                }
            }
        }
        ```