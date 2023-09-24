# Chapter 4
* Ownership is a Rust alternative to **Garbage Collection** in languages like C# & Java, i.e.; it is a form of memory management.

|Memory Management Techniques | Pros  | Cons|
|--|:--:|--:|
|Garbage Collection  |  Error Free + Faster to Write |  No Control Over Memory + Slower/ Unpredictable Runtime Performance + Larger Program Size  |
|Manual Memory Management    |  Control Over Memory + Faster Runtime + Smaller Program Size | Error prone + Slower write time    |
| **Ownership Model** | Control Over Memory + Error free + Faster runtime + Smaller program size   | Slower write time + Learning curve |

* Rust is a memory-safe, systems programming language. Memory safety is achieved via compile-time checks.

*  *  *  
  
## Ownership Rules
1. Each value in Rust has a variable that's called its **owner**.
2. There can only be one owner at a time.
   ```rust
            let s1: String = String::from("hello"); //Memory Allocated on heap
            let s2: String = s1; // Move of ownership from s1 to s2
    ```
    * Ownership transfer via function call on dynamically allocated String.
    ```rust
    fn main(){ 
        let s1: String = String::from("hello");
        takes_ownership(some_String: s1); // ownership transferred from s1 to some_String 
        println!("{}", s1); // Error here "s cannot be borrowed after a move"
    }
    fn gives_ownership(some_String: String){
        println!("{}", some_String);
    } // value "some_String" destroyed after scope
    ```
   * When *s1* is not accessed a compile time error is thrown, as s1 is no longer the owner of the String value "hello". Rust by default **moves** a value during assignment. To clone a value use the following:
   ```rust 
            let s1: String = String::from("hello");
            let s2: String = s1.clone();
   ```
   * Scalar datatypes like Integers, and Booleans are copied by default and not moved.

3. When the owner goes out of scope, the value will be dropped. 

* * *
* While C++ requires us to use the **new** and **delete** keywords to allocate space on the heap and deallocate, Rust does it automatically when scope is exited.

*  *  *
## References 

* To use variables without taking ownership use **References**.
    ```rust
        fn main(){
            let s1: String = String::from("hello");
            let (s2: String, len: usize) = calculate_length(s1);
            println!("The length of '{}' is {}.", s2, len);
        }
        fn calculate_length(s: &String) -> usize {
            let length: usize = s.len();
            length
        }
    ```
    * References are immutable by default. Use mutable references as follows:
        ```rust
            fn main(){
                let mut s1: String = String::from("hello");
                change(some_string: &mut s1);
            }
            fn change(some_string: &mut String){
                some_string.push_str(string: ", world");
            }
        ```
    * Can only have one mutable reference in any given scope, this prevents data race.
        ```rust
            let s: String = String::from("hello");
            let r1: &String = &s;
            let r2: & String = &s; // throws error
        ```
    * Can have multiple immutable references in any given scope.
        ```rust
        let s: String = String::from("hello");
        let r1: &String = &s;
        let r2: &Atring = &s;
        ```
    * Cannot have a mutable refefrence if an immutable reference already exists.
        ```rust
        let mut s: String = Strin::from("hello");
        let r1: &String = &s;
        let r2: &String = &s;
        let r3: &mut String = &mut String; //throws error
        ```

* * *
## The Slice Type
*  Slices let you reference a contiguous block data within a collection instead of referencing the entire collection. Like references, slices do not take ownership.
    * String literal is a string slice.
    ```rust
        fn main(){
            let s: &str = "hello world";
            let word: &str = first_word(s);
            println!("The first word is : {}", word);
        }
        fn first_word(s: &str) -> &str{
            let bytes: &[u8] = s.as_bytes();

            for(i: usize, &item: u8) in bytes.iter().enumerate(){
                if item==b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }
    ```
    * Using Slice in Array
    ```rust
        fn main(){
            let a: [i32; 5] = [1, 2, 3, 4, 5];
            let slice: &[i32] = &a[0..2];
    ```