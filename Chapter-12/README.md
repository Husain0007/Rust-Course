# Chapter 12
 Preparing a command line application inspired by grep(**G**lobally search a **R**egular **E**xpression and **P**rint).

 ## Version 1:  
* Create a new Rust project
```console 
cargo new minigrep
```
* Create binary crate ie; main.rs that handles command-line arguments and file parsing.
    *  Using: **env**, **process**, & **error::Error** modules from **std** library.
    * Create **Config** struct to handle **query** and **filename**
    * Use struct constructor *new* to return a **Result** type 
    * Use *closures* to handle errors arising from missing files or inappropiate number of command-line arguments. 
  *  *  *  

 ## Version 2:
 * Since *main.rs* is quite bloated, **run()** function and **Config** struct have been migrated to *lib.rs* file.  
 * **run()** and **Config** are made public to be accessed by *main.rs*. 
    *   Access via `use minigrep::Config` & `use minigrep::run`
 * Practice test driven development in *lib.rs*.  
    * Define tests module: `#[cfg(test)]`
 * Use *Lifetime* declaration to tie lifetime of reference to lifetime of the input parameter.
    * `pub fn search <'a> (query: &str, contents: &'a str) -> Vec<&'a str> { ... }`
 * Account for case sensistive and insensitive search.
    * Use environment variables from *env* module.
    * Modify **Config** structs' function to use environment variable.
 * In *main.rs* display errors to terminal using `eprintln!` instead of `println!`. Doing so prevents errors from being saved to **output.txt**.
    ```console 
        cargo run to poem.txt > output.txt 
    ```

