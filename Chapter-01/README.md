# Chapter 1

* Rust is systems-programming language with a compiler that ensures stability through feature additions and refactoring. 
* Compiling and running a program in Rust.
```
rustc file.rs
./file
```
* Using the Rust Dependency Manager **Cargo** to create, compile and run projects.
   
``` 
cargo new project_name
cd project_name
cargo build
cargo run
```

* `cargo build` produces a new *Cargo.lock* file that specifies all dependencies. A `target` directory is also created which has a further `debug` directory.
