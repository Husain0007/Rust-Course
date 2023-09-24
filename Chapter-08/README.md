# Chapter 8 : Common Collections
* While other data structures store only a specific data type at a time, collections can contain multiple data types. Collections are also, unlike built-in data structures like arrays and tuples, stored on the heap.
* Collection types are:
    * Vectors
    * Strings
    * Hashmaps
* * *
## Vectors
* Since Collection types are dynamically allocated their values are dropped once outside of scope.
    ```rust
        fn main() {

            let mut v: Vec<i32> = Vec::new(); 
            // dynamically allocated so we have to specify type
            v.push(1);
            v.push(2);
            v.push(3);
            {
                let v2 = vec![1, 2, 3]; // Using vec! macros to initialize vector with values
            } // value of v2 is dropped outside scope
            println!("{}",v[0]); // no error as v is in scope
            println!("{}",v2[0]); // throws error: v2 doesn't exist in scope

        }
    ```
* Accessing elements by index at runtime can lead to *index out of bounds* error. To get aroung this problem use the **get()** method.
    * The **get()** method returns an *Option*, i.e.; an expression with *Some* or *None* value. 
    ```rust
        match v.get(index: 2){
            Some(third: &i32) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    ```
* Rust prevents us from having mutable and immutable references at the same time. Either we can have multiple immutable references or one mutable reference. When we have an immutable reference to a collection type, the underlying data should not be changed.
    ```rust
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2]; // immutable reference to vector "v"
        v.push(6); // ILLEGAL: a mutable reference is taken to push a new value to vector "v"
    ```
* * *
## Strings
* Strings are stored as a collection of UTF-8 encoded bytes.
* UTF-8 is a varibale-width character encoding 
* ASCII encodes characters using 7-bits (each character is represented by 1 byte) which cover only the English language and some special characters.
* Unicode has a much wider array of characers. It is also backwards compatible with ASCII, i.e.; first 128 characters of Unicode are ASCII characters.
    ```rust
        // String are stored as a collection of UTF-8 encoded bytes
        let s1: String = String::new();
        let s2: &str = "initial contents"; // string slice
        let s3: String = s2.to_string(); // create String from string slice
        let s4: String = String::from("initial contents"); // create String with "from"
         let hello = String::from("नमस्ते"); // Use other languages.
    ```
    * Concatenating strings using one change of ownership and one reference to save on memory.

    ```rust
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3: String = s1+ &s2;   // memory saving concatenation.
        let s4 = &s2;
        // moving ownership from s1->s3 and appending to s3 all characters in s2
        println!("{}",s2); //LEGAL: since only an immutable reference to s3 exists. 
                        // ownership remains with s2
        println!("{}", s1); //ILLEGAL: since ownership has been moved
    ```
    * Retaining ownership in s1 and s2 with *format!* macro.
 
    ```rust
        let s1: String = String::from("Hello ");
        let s2: String = String::from("world!");
        let s3: String = format!("{}{}", s1, s2);
    ```
    * **Indexing into a String :** In UTF-8 there are 3 ways to look at a string.
        
        ```rust
            //Consider namaste
            let hello = String::from("नमस्ते");
        ```
        * As Bytes
            ```rust
                for b in hello.bytes(){
                    println!("{}", b);
                }
            ```
            ```cmd
            [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,224, 165, 135]
            ```
        * As Scalar Values 
            ```rust
                for c in "नमस्ते".chars(){
                    println!("{}", c);
                }
            ```
            ```cmd
                ['न', 'म', 'स', '्', 'त', 'े']
            ```
        * As Grapheme Clusters
            * To keep Rust lean. The functionality to iterate over Grapheme Clusters needs to be imported into the `Cargo.toml` file under `[dependendies]` as `unicode-segmentation = "1.7.1"`.
            ```rust
                for g in "नमस्ते".graphemes(true){
                    println!("{}",g);
                    }
            ```
            ```cmd
                ["न", "म", "स्", "ते"]
            ```
* * *
## Hash-Maps

* Used to store key value pairs.
* Bring into scope with `use std::collections::HashMap`.
    ```rust
        let blue = String::from("Blue");
        let yellow = String::from("Yellow");

        let mut scores: HashMap<String, i32> = HashMap::new();

        scores.insert(blue, 10); 
        scores.insert(yellow, 50);
        // ownership of strings is passed to the hashmap
        println!("{}",blue); //ILLEGAL!
    ```
    * We can pass strings as reference to HashMap but that would require using `Lifetimes`.
* Getting count of strings and storing it in a HashMap.
    ```rust
        let text = "hello world wonderful world";

        for word in text.split_whitespace(){
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    ```
    * The `.or_insert()` methiod returns a reference. Using the dereference `*` operator we can increment the value starting from `0` and get a count for each word.