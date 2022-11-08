use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new(); // dynamically allocated so we have to specify type

    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1, 2, 3]; // Using vec! macros to initialize vector with values
    } // value of v2 is dropped outside scope

    let third = &v[2];
    //v.push(9); => Mutable reference is illegal since "v" already has an immutable reference in "third" 

    println!("The third element is {}", third);

    // Handle values by index at runtime
    match v.get( 20){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //Iterating using for-in loop with immutable reference for each element
    for i in &v { 
        println!("{}", i);
    }
    //Iterating using for-in loop with mutable reference and dereference operator (*)
    for i in &mut v {
        *i += 50;  
        println!("{}", *i);
    }

    enum SpreadSheatCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![ SpreadSheatCell::Int(3), SpreadSheatCell::Text(String::from("blue")), SpreadSheatCell::Float(10.12),];
    
    match &row[1] {
        SpreadSheatCell::Int(i) => println!("{}",i),
        _ => println!("Not a Integer!")
    };

    let mut s = String::from("foo");
    s.push_str("bar"); // Way 1: Append a string
    s.push('!');
    println!("{}",s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3: String = s1+ &s2;   // memory saving concatenation.
    let s4 = &s2;
    // moving ownership from s1->s3 and appending to s3 all characters in s2
    println!("{}",s2); //LEGAL: since only an immutable reference to s3 exists. 
                       // ownership remains with s2
    //println!("{}", s1); //ILLEGAL: since ownership has been moved
    let s1: String = String::from("Hello ");
    let s2: String = String::from("world!");
    let s3: String = format!("{}{}", s1, s2);
    println!("{}",s3);

    // Indexing into a string represented using UTF-8 is not straight-forward
    // Index as bytes, chars, grapheme clusters
    for b in "नमस्ते".bytes(){
        println!("{}", b);
    }
    for c in "नमस्ते".chars(){
        println!("{}", c);
    }
    // Functionality to iterate over grapheme clusters has to be imported.
    for g in "नमस्ते".graphemes(true){
        println!("{}",g);
    }

    // HashMaps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10); 
    scores.insert(yellow, 50);
    // ownership of strings is passed to the hashmap
    // println!("{}",blue); //ILLEGAL!
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // Overwriting values in HashMaps
    scores.insert(String::from("Orange"), 12);
    scores.insert(String::from("Orange"), 21);

    //Using or_insert to check if entry exists before writing value
    scores.entry(String::from("Yellow")).or_insert(30);
    // "Yellow" doesn't exist so (Yellow, 30) is added
    scores.entry(String::from("Yellow")).or_insert(40);
    // "Yellow" does exist so (Yello, 30) isn't changed.

    let mut map: HashMap<&str, i32> = HashMap::new();

    let text = "hello world wonderful world";

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
