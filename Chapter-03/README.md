# Chapter 3
* Reviewing various Data Types, Functions, Control Flow Statements, and Comment types.
* Data Types  
    * Scalar Types
        * Integers
        * Floating-Point Numbers
        * Booleans
        * Characters
    * Compound Types
        * Tuples
        * Arrays
    * Functions
        * Last expression in block is an implicit return type
        * return type must be specified with **->** in function declaration.
        ```rust
                    fn function(x: i32, y: i32) -> i32 {
                            x+y
                        }
        ```
        * Rust code is of two types **Statements**(no return type) & **Expressions**(with return type).
    * Control Flow Statements
        * If-Else Blocks
            * Inline if-else block with **let** keyword
            ```rust
                    let condition = true;
                    let number = if condition {5} else {6};
            ```
        * Loop
            * Can have a return type
            ```rust 
                    let result: i32 = loop{...}
            ```
        * While
        * For
    * Comments
        * //
        * /* */
        * Doc Comments