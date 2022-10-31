# Chapter 5
* Structs are used to group related data.
* To make changes to any attribute of a struct after instantiation the entire struct needs to be made mutable.
    ```rust
        struct User{
            username: String,
            email: String,
            age: u32,
            active: bool,
        }
        fn main(){
            let mut husain: User = User {
                email: String::from("syedhusainmustafa@gmail.com"),
                username: String::from("Husian007"),
                active: true,
                age: 25
            };
            let age: u32 = husain.age;
            husain.username = String::from("Husain0008");
        }
    ```