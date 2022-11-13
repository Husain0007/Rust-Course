# Chapter 12: Testing in Rust


## Writing Tests
* While the Rust compile accounts for many errors it cannot check the correctness of our Business logic. To account for that we write tests.

* Create new cargo library:
    ```cmd 
        $ cargo new adder --lib
    ```
* In Rust functions are test if they possess the `#[test]` attribute.
    ```rust
        mod tests {
            #[test]
            fn it_works(){
                assert_eq!(2+2, 4);
            }
        }
    ```
* To run all tests defined inside the tests module.
    ```cmd
        $ cargo test
    ```
* Each test is run in a new thread. If the main thread sees any of the test thread fail then the test fails. In Rust a test fails if something inside the test panics. 
    ```rust
        mod tests {
            #[test]
            fn it_works(){
                assert_eq!(2+2, 4);
            }
            #[test]
            fn failing_test() {
                panic!("Make this test fail");
            }
        }

    ```
* In tests the `assert!` is used to check if an expression evaluates to true.
    ```rust
        struct Rectangle{
            width: u32,
            height: u32
        }
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self. width > other.width && self.height > other.height
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*; // To gain access to modules defined in parent scope.
            #[test]
            fn larger_can_hold_smaller() {
                let larger: Rectange = Rectange {
                    width: 8,
                    height: 7,
                };
                let smaller: Rectange = Rectangle {
                    width: 5,
                    height: 1,
                };
                assert!(larger.can_hold(&smaller));
            }
        }
    ```
* The macro `assert_eq!` checks if the two inputs are equal.
    ```rust
        pub fn add_two(a: i32) -> i32 {
            a + 2
        }
        mod tests {
            use super::*;
            #[test]
            fn it_adds_two() {
                assert_eq!(4, add_two(2));
            }
        }
    ```
* The macro `assert_ne!` checks if two inputs are not equal. Both parameters passed into `assert_ne!` or `assert_eq!` have to implement the `PartialEqual` & `Debug` traits. All primitive types implement these traits by default.
* * *

## Organizing Tests into Integration Tests & Unit Tests
* When we execute `cargo test` in the terminal by default all test run in parallel threads.
* Passing Options to `cargo test`:
    ```cmd
        $ cargo test -- --test-threads 1
    ```
    * Above command is used to run tests in a sequence. This is not usually desired as testing takes longer time. However in situations when tests modify a file on disk processing tests in parallel would lead tests to fail.
* Running a subset of tests by specifying test names.
    * Consider a set of tests given below.
    ```rust
        pub fn add_two(a: i32) -> i32 {
            a + 2
        }
        #[cfg(test)]
        mod tests{
            use super::*;
            #[test]
            fn add_two_and_two() {
                assert_eq!(4, add_two(2));
            }
            #[test]
            fn add_three_and_two(){
                assert_eq!(5, add_two(3));
            }
            #[test]
            fn one_hundred() {
                assert_eq!(102, add_two(100));
            }
        }
    ```
    * To only run the `one_hundred` test.
        ```cmd
            $ cargo test one_hundred
        ```
    * To only run tests with prefix `add`. 
        ```cmd
            $ cargo test add
        ```
        * This will run both the `add_two_and_two` and `add_three_and_two` tests.
    * To ignore a test specify the `#[ignore]` attribute before the test function definition.
        ```rust 
            mod tests{
                #[test]
                fn it_works(){
                    assert_eq!(2+2, 4);
                }
                #[test]
                #[ignore]
                fn expensive_test(){
                    // code that takes an hour to run
                }
            }
        ```
       * A standard `cargo test` command will ignore the `expensive_test`.
       * To run only the test with `#[ignore]` attribute.
            ```cmd
                $ cargo test -- --ignored
            ```
### **Test Organization**
* There are two types of tests:
    * Unit Tests : test one module in isolation. Could test private interfaces.
    * Integration Tests: tests are external to the library. Used to test the public interface of the library.
* In Rust Unit Tests live in the same library as the product code. So far we have examined unit tests. Child `tests` module can access all functions (including private ones) defined inside the parent module.
    ```rust
        fn internal_adder(a: i32, b: i32) -> i32 { 
            // This function is private by default
            a + b
        }
        pub fn add_two(a: i32) -> i32 {
            internal_Adder(a, b: 2)
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn internal() {
                assert_eq!(4, internal_adder(2,2));
            }
        }
* **Integration Tests** live inside a folder called `tests` in the project root folder.
    ```
    project
    ├──src
    │   └── lib.rs
    ├── target
    │   └── debug
    └── tests
        └── integration_test.rs
    ```
    * Cargo converts each file inside the `tests` directoy into a crate.
    * While `#[cfg(test)]` needs to be specified at the top of a unit test module defined inside the `src` folder. It does not need to be specified inside the `tests` folder.
    ```cmd
        $ cargo test --test integration_test
    ```
    * Is used to just test the tests defined inside the `integration_test.rs` file inside the `tests` folder.
    * We cannot test a binary crate(main.rs) with integration tests, only library crates (lib.rs).
