# Chapter 6
* Whereas structs allow grouping of related fields and adata, enums offer a way of saying a value is one of a possible set of values.
* Enums can be used inplace of structs as variants of an enum type. 
    * For example:
        ```rust
            enum Message {
                // Enum variants
                Quit,
                Move {x: i32, i:32},
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            // Alternatively use structs as below
            struct QuitMessage; //unit struct
            struct MoveMessage {
                    x: i32,
                    y: i32,
            }
            struct WriteMessage(String); //tuple struct
            struct ChangeColorMessage(i32, i32, i32); //tuple struct
        ```
    * The advantage of using enums instead of four structs in above case is that the different values associated with the Enum Variants despite belonging to different scalar types fall under the same user-defined type known as *Message*. 
    * Enums, like Structs, can also have associated functions and methods in a separate **impl** block.
        ```rust
            enum Message {
                // Enum variants
                Quit,
                Move {x: i32, i:32},
                Write(String),
                ChangeColor(i32, i32, i32),
            }
            impl Message {
                fn some_function()
                {
                    println!("Hello World");
                }
            }
        ```
*  *  *
## The Option Enum
* In Rust there are no *null* values, instead we have the option enum.
* The option enum has only two variants: 
    * Some
    * None
        ```rust
            enum Option<T> {
                Some(T),
                None,
            }
        ```
* If we have a value that could potentially not exist, then we would have to wrap it in the option enum.
* The option enum is included in our progrma scope by default, i.e.; the option enum doesn't have to be implemented by the user.  

    ```rust
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
    ```
* To extract value from **Some** variant use the **unwrap_or()** function which also takes a default value incase the value is None

    ```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    println!("{}",x+y.unwrap_or(0)); // Output: 5+5=10

    // When z is None
    let z: Option<i8> = None;
    println!("{}",x+z.unwrap_or(0)); // Output: 5+0=5
    ```
* * *
## Match
* Match lets you compare a value against a set of patterns, like literals, variables, wildcards, etc.
* Match is exhaustive.
    * All values must be mentioned to avoid errors.
    * The placeholder **_** can be used to match for some default value
    ```rust
        fn main(){
            let five: Option<i32> = Some(5);
            let sx: option<i32> = plus_one(five);
            let none: Option<i32> = plus_one(None);
        }
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x{
                //None => None,
                Some(i) => Some(i+1),
                _ => None,
            }
        }
    ```
* * *
## If-Let Syntax
* If we are working with enums but are only concerned with checking for one specific value, then match expressions are not optimal as they are too verbose. 
    ```rust
        fn main(){
            let some_value: Option<i32> = Some(3);
            match some_value {
                Some(3) => println!("three"),
                _ => (),
            }
        }
    ```
* We can use an if-let expression instead. Unlike the match expression we do not have to account for all values, i.e.; do not need to specify other patters or a default **_** pattern.
    ```rust
        fn main(){
            if let Some(3) = some_value {
                println!("three");
            }
        }
    ```
