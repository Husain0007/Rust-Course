# Chapter 10: Generics, Traits, & Lifetimes.

## Generics
* Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.
* Consider the example fo obtaining the largest number from a generic list.
    ```rust
        fn get_largest<T>(number_list: Vec<T>) -> T {
            let mut largest = number_list[0];
            for number in number_list {
                if number > largest {
                    largest = number;
                }
            }
            largest
        }
    ```
    * The above function throws an error at the ```number > largest``` comparision operation as the generic type `T` may be a type, say Struct or Object, that cannot be compared. In such a situation we need to introduce **traits** which restrict the types that can be used in the function call.
    * We specify that `T` must be a type that can be ordered by `PartialOrd` and a type that can be copied by `Copy`. Only primitive types can be copied.
    ```rust 
        fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
            let mut largest = number_list[0];
            for number in number_list {
                if number > largest {
                    largest = number;
                }
            }
            largest
        }
    ```
* Can use generics with Structs as well.
    ```rust
        struct Point<T, U> {
            x: T, 
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        fn main(){
            let p1 = Point{x:5, y:10};
            let p2 = Point{x:5.0, y: 10.2};
            let p3 = Point{x:5, y:10.2}; 

            let p4 = p1.mixup(p2);
            println!("p4.x = {}, p4.y = {}", p4.x, p4.y);
        }

    ```
* * *
## Traits
* Traits allow us to define the shared behavior, i.e.; a shared set of methods between types.
    ```rust
        pub struct NewsArticle {
            pub author: String,
            pub headline: String,
            pub content: String,
        }
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{} by {}", self.headline, self.author)
            }
        }
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        pub trait Summary {
            fn summarize(&self) -> String;
        }
    ```
* Traits as parameters. 
    ```rust
        pub fn notify (item: &impl Summary){
            println!("Breaking news! {}", item.summarize());
        }
    ```
    * Only items that implement the `Summary` trait will be allowed as input parameter to the `notify` function.
### Trait Bounds
```rust
    pub fn notify<T: Summary + Display>(item: &T){
    println!("Breaking news!{}", item.summarize());
    }
    // #1 Multiple Traits
    pub some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) ->i32{
        //...
    }
    // #2 Alternate Representation 
    fn some_function<T, U>(t: &T, u: &U) -> i32 
        where T: Display + Clone, // traits implemented  
              U: Clone + Debug
        {
            //..
        }
```
* Return types that implement certain traits. 
    ```rust
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "As you already know..."
                ),
                reply: false,
                retweet: false,
            }
        }
        fn main() {
            println!("{}", returns_summarizable().summarize());
        }
    ```
* Returning types that implement a certain trait instead of a concreate type is very useful inside of `Closures` and `Iterators`. 
* Using Trait-Bounds to conditionally implement methods.
    ```rust
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl <T> Pair<T> { // this impl block wors for any Pair struct
            fn new(x: T, y: T) -> Self {
                Self {x, y}
            }
        }
        impl<T: Display + PartialOrd> Pair<T> { // Using Trait-Bounds
        // Only types T that has "Display" and "PartialOrd" trait can implement the functionality inside this impl block.
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The larges member is y = {}", self.y);
                }
            }
        }
    ```
### Blanket Implementation
* Implement a trait on a type that implements another trait.
    ```rust
        impl<T: Display> ToString for T {
            // --snip--
        }
    ```
* * *
## Rust Lifetimes 
* Dangling Reference: A reference that points to invalid data.
* Rust uses a `Borrow Checker` at compile-time to check if all borrowed values/ references are valid.
    ```rust
        fn main(){                                          // a
            let r: &i32;                                    // a
            {                   //  b                       // a
                let x: i32 = 5; //  b                       // a
                r = &x;         //  b                       // a
            }                   //  b                       // a
            println!("r: {}", r); // compile time error     // a
        }
    ```
    * The above code will not be allowed to run in Rust as `r` would hold an invalid value after the scope **b** ends.
* Lifetime of a variable refers to how long it lives for.
* Generic Lifetime Annotation (also known as just `Lifetimes`): describes the relationship between the lifetimes of multiple references and how they relate to each other. This does not change the *actual* lifetime, only describes it.
* Generic Lifetime Annotations start with an `'` apostrophe symbol followed by name of the lifetime.
    ```rust
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    ```
    * The use of `<'a>` makes it clear to the Borrow-Checker that the lifetime of the return type will be same as that of the shortest lifetime between `x` and `y` which both use the same Generic Lifetime Annotation **a**. Here `x` and `y` are the input parameters. For the below main block the Borrow-Checker will return the address of the longest string-slice from amongst `x` (a.k.a. string1).

    ```rust 
     fn main() {
        let string1: String = String::from("abcd");
        let string2: String = String::from("xyz");

        let result: &str = longest(x: string1.as_str(), y: string2.as_str());
        println!("The longest string is {}", result);
    }
    ```
* The lifetime of a return type in a function always has to be tied to the lifetime of atleast one of the input parameters. This is because we cannot pass back reference to data created with the scope of the function. Such a reference would be invalid when the scope ends, hence the returned address would be invalid. A work around to this is just to return the type instead of it's reference, this just transfers ownership when the variable inside the function scope is destroyed.
    ```rust
        fn main(){
            let string1: String = String::from("abcd");
            let string2: String = String::from("xyz");
            let result: String;
            result = longest(x: string1, y: string2);
            println!("The longest string is {}", result.as_str());
        }
        fn longest<'a>(x:String, y:String) -> String {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    ```
* To use references inside Structs we have to specify Lifetime Annotations.
    ```rust
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        fn main(){
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            let i = ImportantExcerpt {
                part: first_sentence,
            };
            println!("{}", i.part);
        }
    ```
* **Lifetime Elison Rules :** The Compiler follows three rules in automatically determining the lifetime of variables. If it is unable to ascertain the lifetime using these three rules, then we have to manually specify them.
    1. Each parameter that is a reference gets its own lifetime parameter.
    2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` the lifetime of `self` is assigned to all output lifetime parameters. 
* Lifetime Annotations are a type of Generics, so in `impl` blocks they have to be encorporated in the same way as a Generic Expression.
    ```rust
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        impl<'a> ImportantExcerpt<'a>{
            fn return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        } 
    ```
    * As per Rule-3 we won't need to specify a Lifetime Annotation Reference to self when returning `self.part` as the Rust Compiler automatically assigns the lifetime of `self` to all output lifetime paramters, i.e.; to all return statements.
* **Static Lifetime :** The reference can live as long as the duration of the program. All string literals have a static lifetime.
    ```rust
        fn main(){
            let s: &'static str = "I have a static lifetime.";
        }
    ```
* * *
## Putting it all Together 
* Following snippet encorporates Generics, Traits, & Lifetimes.
    ```rust
        use std::mt::Displayy;
        
        fn longest_with_an_announcement<'a, T>(
            x: &'a str, 
            y: &'a str,
            ann: T,
            ) -> &'a str
            where 
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len > y.len() {
                x
            } else {
                y
            }
        }
        fn main() {}

    ```