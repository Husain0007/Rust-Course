# Chapter 12
 Preparing a command line application inspired by grep(**G**lobally search a **R**egular **E**xpression and **P**rint).

 Version 1:  
* Create a new Rust project
```console 
cargo new minigrep
```
* Create binary crate ie; main.rs that handles command-line arguments and file parsing.
  *  *  Using: **env**, **process**, & **error::Error** modules from **std** library.
  *  * Create **Config** struct to handle **query** and **filename**
  *  * Use struct constructor *new* to return a **Result** type 
  *  * Use *closures* to handle errors arising from missing files or inappropiate number of command-line arguments. 
  *  *  *  