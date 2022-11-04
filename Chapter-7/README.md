# Rust Module System Management
* To create a new **package** in Rust.
        
    ```cmd
        $ cargo new package_name
    ```
* A **package** stores **crates**.
    * There are two types of **crates**. 
        * Binary Crates i.e.; executables.
        * Library crates i.e.; code that can be used by other programs.

* **Crates** are defined inside the **Cargo.toml** file. 
    * When we create a new package, by default a **main.rs** file is created inside the src directory. This file is the default **crate-route**, i.e.; the source file the Rust compiler starts at when creating the binaries.   
    * If **lib.rs** is defined in the root of the src directory then Rust will create a library crate with the same name as the package. Here lib.rs will be the crate-root.
    
* **Crates** contain **modules**, which can be make **public** or **private**.

* **Public** parts of a crate's module can be accessed by specifying a **path** inside the external file that needs access. 

* * *
## Creating a Restaurant Package

* Specify lib flag to initialize package with a library crate

    ```cmd
        $ cargo new --lib restaurant/
        $ cd restaurant/
    ```
* Now `restaurant/src` will have **lib.rs** instead of **main.rs**.

        crate
        └── front_of_house
            ├── hosting
            │   ├── add_to_waitlist
            │   └── seat_at_table
            └── serving
                ├── take_order
                ├── serve_order
                └── take_payment

    * The library crate will have a module called **front_of_house** which will have two sub-modules, and each submodule with have a set of functions.

    ```rust
        mod front_of_house{
            mod hosting {
                fn add_to_waitlist() {}
                fn seat_at_table() {}
            }

            mod serving {
                fn taken_roder() {}
                fn serve_order() {}
                fn take_payment() {}
            }
        }
    ```

* Specifying Path to a module function.
    * By default child modules are private.

    ```rust
        mod front_of_house{
            pub mod hosting {
                pub fn add_to_waitlist() {}
                fn seat_at_table() {}
            }
        }

        pub fn eat_at_restaurant() {
            //Absolute Path
            crate::front_of_house::hosting::add_to_waitlist();
            //Relative Path
            front_of_house::hosting::add_to_waitlist();
        }
    ```
* Use `super` to reference functions defined in parent module.

* Use the `use` keyword to bring functions into scope. Usually we just bring upto the parent module in scope.
    ```rust
        mod front_of_house{
            pub mod hosting{
                pub fn add_to_waitlist() {}
            }
        }

        use self::front_of_house::hosting;

        pub fn eat_at_restaurant() {
            hosting::add_to_waitlist();
            hosting::add_to_waitlist();
        }
    ```
* If two functions or modules share a name use the `as` keyword to change it's reference name.
    ```rust
        use std::fmt::Result;
        use std::io::Result as IoResult;

        fn function1() -> Result {
            Ok(())
        }
        fn function2() -> IoResult<()> {
            Ok(())
        }
    ```
* Use nested paths when importing many submodules from same module
    ```rust
        use rand;
        use rand::Rng;
        use rand::CryptoRng;

        // Alternatively
        use rand::{self,Rng,CryptoRng}

        use std::io::*; 
        // use glob operator to bring all public items in scope

    ```
* To reduce code size in a file we can move the content of a module or sub module to a ".rs" file with the same name as module.
    * Consider the following folder heirarchy

        ```
        src
        ├── front_of_house
        │   ├── hosting.rs
        │   └── serving.rs
        └── front_of_house.rs
        │
        └── lib.rs
        ```
    * The module content of "front_of_house" is stored insider a file called `front_of_house.rs`
    * The sub-module content of "hosting" and "serving" needs to be stored inside files called `hosting.rs` and `serving.rs` which live insider a folder called `front_of_house`.