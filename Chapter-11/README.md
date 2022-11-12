# Chapter 12: Testing in Rust

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
