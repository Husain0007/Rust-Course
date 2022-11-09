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
* Using `Closures` instead of nested match expressions.
    ```rust
        fn main(){
            let f: Result<File, Error> = File::open(path: "hello.txt");

            //Using Closures
            let f: File = File::open(path: "hello.txt").unwrap_or_else(op: |error: Error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create(path: "hello.txt").unwrap_or_else(op: |error: Error| {
                        panic!("Problem creating the file: {:?}", error);
                    })
                    }    else {
                        panic!("Problem opening the file : {:?}", error);
                    }
            })
        }
    ```
* * *
## Error Propagation
* Return error to the caller function, which then decides what to do with that error.
    ```rust
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn read_username_from_file() -> Result<String, io::Error> { 
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s){
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
    ```
* Using `?` works similar to `unwrap` and `expect` methods.
    * Incase of file retrieval, if file exists it is returned to the variable, if not, then the function ends early with an error
    ```rust
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn read_username_fro_file() -> Result<String, io::Error> {
            let mut f: File = File::open(path: "hello.txt")?;

            let mut s: String = String::new();

            f.read_to_string(buf: &mut s)?;
            Ok(s)
        }
    ```
    * Above function can be further simplified by *chaining method calls*.
    ```rust
        use std::fs::File;
        use std::io;
        use std::io::Read;
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s: string = String::new();
            File::open(path: "hello.txt")?.read_to_string(buf: &mut s)?;
            Ok(s)
        }
    ```
    * The `?` operator can only be used in a function that returns `Result` or `Option`.
* * *
* Results Enum coupled with Error Propagation should be preferred over using the panic macro.