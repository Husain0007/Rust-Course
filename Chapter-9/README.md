# Chapter 9: Error Handling

## Using Panic!
* If the program fails in a way that is unrecoverable then call the `panic!` macro to immediately quit the program.
* Use `RUST_BACKTRACE=1` environment variable to trace the step-by-step function calls leading up to the eventual error.
    ```rust
    fn main(){
        a()
    }
    fn a(){
        b()
    }
    fn b() {
        c(num: 22)
    }
    fn c(num: i32){
        if num ==22 {
            panic!("Don't pass in 22!");
        }
    }
    ```
    * Run the program as: `RUST_BACKTRACE=1 cargo run`
* * *
## The Results Enum
* This is used to handle recoverable errors that don't crash the program.
* Like the *Option* enum, the result enum has two variants.
    ```rust
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    ```
* Use a match expression to check the corresponding Result enum value from an `open` file operation. Shadowing is used to re-declare variable `f`.
    ```rust
        use std::fs::File;

        fn main(){
            let f = File::open("hello.txt");

            //Shadowing 
            let f = match f {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
        }
    ```
    * If result is `Ok` we bind `f` to the `file` variable and return it.
* To check for what specific error occurred we can use nested match expressions with variants of `ErrorKind`. In the block below `hello.text` is created if it isn't available.
    ```rust
        use std::fs::File;
        use std::io::ErrorKind;
        
        fn main(){
            let f = match f {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.text"){
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    },
                    other_error =>{
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                },
            };
        }
    ```