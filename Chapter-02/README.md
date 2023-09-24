# Chapter 2

* Preparing a guessing game that makes use of the **rand** and **cmp** modules.
*   *   *  
## Shadowing
* Refers to changing type of a variable while maintaining the same variable name.
### Shadowing : Version 1  
```rust
    let guess: u32 = guess.trim().parse().
                expect("Please type a number");
```
* The above form of *Shadowing* is limited in that the program execution ends when a non-numeric string is passed to *guess*.

### Shadowing : Version 2
```rust
    let guess: u32 = match guess.trim().parse()
    {
        Ok(num: !) => num,
        Err(_) => continue,
    };
```
* The above implementation relies on the fact that *parse()* returns a **Result** type enum with two entries **Ok** and **Err**.
* The **match** statement is used to account for both **Result** scenarios. 
   * Here **Ok** is returned for an input string of type **u32**.
   * Here **Err** is returned for any other type, where **_** is used as a catch-all term.