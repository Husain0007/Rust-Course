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
# Rust Lifetimes 

