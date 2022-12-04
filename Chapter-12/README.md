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

   * * *
 ## Version 3:
   * Substituting array reference in `Config` block of `lib.rs` with an Iterator.To accomplish this we change from using `collect()` to create a list, to instead just using the iterator in `main.rs`  
      ```rust
         let config = Config::new(env::args()).unwrap_or_else(|err|{
               eprintln!("Problem parsing arguments: {}", err);
               process::exit(1);
            }); // Use this^ instead of below
         //******//
         let args: Vec<String> = env::args().collect();

         let config = Config::new(&args).unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
         });
      ```
   * Simplify the search function to use iterators instead of for loops with vectors. First create an iterator by calling `contents.lines()`, filter the desired lines by examining all iterable values for the `query` and then call `collect()` to combine all valid iterable values into a vector of type `string`.
      ```rust
            pub fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
               contents: &str
                  .lines(): Lines
                  .filter(|line: &&str| line.contains(query)): imp Iterator<Item = &str>
                  .collect()
            }
      ```
   * As Rust follows the zero level abstraction principle, by using iterators over loops we don't observe any difference in performance. Using iterators however is preferred since it unlocks a higher of abstraction and access to functions such as `filter()` and `collect()`.


