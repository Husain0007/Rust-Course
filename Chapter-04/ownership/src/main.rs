fn main() {
    /*Ownership Rules
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.  */
    { // s is not valid here, it's not yet declared.
        let s: &str = "hello"; // s is valid from this point forward
                            // do stuff with s.
    }    // this scope is now over, and s is no longer valid.

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1.clone();

    let s3 = s1; //Move(not shallow copy)
    println!("{}, world!", s3);

    takes_ownership(s3);
    
    //println!("{}, world!", s3); => Error: s3 cannot be borrowed after a move

    // Using Reference based function below
    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);

    // Getting slice index
    let mut s: String = String::from("hello world");
    let s2 = "hello world"; // string literal is a string slice

    let word = first_word(s2);
    //s.clear();
    println!("the first word is : {}", word);

    // Array slice
    let a = [1, 2, 3, 4, 5];
    let slice= &a[0..2];

}

fn takes_ownership(some_String: String){
    println!("{}", some_String);
}

/* Using References and not taking ownership of underlying value */
fn calculate_length(s: &String) -> usize{
    let length = s.len();
    length
}

/* Using Slice type */
// returns a string slice
fn first_word(s: &str) -> &str{
    let bytes: &[u8] = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item ==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
} 
