# Chapter 13: Closures & Iterators


## Closures

* Closures are anonymous functions. 
* They can be stored as variables and passed around.
* They can also be passed in as input parameters to a function.

* Consider an expensive function.
    ```rust
        fn simulated_expensive_calculation(intensity: u32) -> u32 {
            println!("calculating slowly ...");
            thread::sleep(Duration::from_secs(2));
            intensity
        }
    ```

* Consider the following function that calls an expensive function multiple times.
    ```rust
        fn generate_workout(intensity: u32, random_number: u32){
            if intensity < 25 {
                println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
                println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
                }
            }
        }
    ```
    * To prevent the expensive function from being called multiple times in each if-else block we will fetch the value from the function once and pass the variable to the respective if-else block.
        ```rust
            fn generate_workout(intensity: u32, random_number: u32){
                let expensive_result = simulated_expensive_calculation(intensity);
                if intensity < 25 {
                    println!("Today, do {} pushups!", expensive_result);
                    println!("Next, do {} situps!", expensive_result);
                } else {
                    if random_number == 3 {
                        println!("Take a break today! Remember to stay hydrated!");
                    } else { 
                        println!("Today, run for {} minutes!", expensive_result);
                    }
                }
            }
        ```
        * This is not desirable either as for `random_number==3` we don't need to run the expensive function, but in the above implementation it is run by default whenever `generate_workout()` is called.
    
    * Lets define a function instead of using a call to the expensive function.
        ```rust
            fn generate_workout(intensity: u32, random_number: u32){
                let expensive_closure= |num: i32| {
                    println!("calculating slowly...");
                    thread::sleep( Duration::from_secs(2));
                    num
                };    if intensity < 25 {
                    println!("Today, do {} pushups!", expensive_closure(intensity));
                    println!("Next, do {} situps!", expensive_closure(intensity));
                } else {
                    if random_number == 3 {
                        println!("Take a break today! Remember to stay hydrated!");
                    } else {
                        println!("Today, run for {} minutes!", expensive_closure(intensity));
                    }
                }  
            }
        ```  
### About Closures
* For Closures we do not have to specify the input and return types, whereas in functions we have to.
* The first type passed into a Closure will be the concreate type for the input parameter.
    ```rust
        let expensive_closures: |...| -> u32 = |num| {
            todo!()
        };
        let s: String = example_closure(String::from("hello")); 
        // ^ "String" concreate type had been assigned to input param of Closure
        let n: String = example_closure(5); // compiler complains here
    ```
* Now we use **memoization** to hold the closure in a struct as well as it's result.
    * The struct uses a generic type `T` which is `Fn` trait bounded. With this bound we can store reference to a closure within a member of the struct. Normal functions also implement the `Fn` trait so we could also store a reference to them inside our struct member variable.
        ```rust
            struct Casher<T>
            where 
                T: Fn(U) -> U,
            {
                calculation: T,
                value: Option<U>
            }
            impl<T> Cacher<T> 
                where T: Fn(U) -> U,
            {
                fn new(calculation: T) -> Cacher<T> {
                    Cacher {
                        calculation,
                        value: None,
                    }
                }

                fn value(&mut self, arg: U) -> U {
                    match self.value {
                        Some(v) => v,
                        None => {
                            let v = (self.calculation)(arg);
                            self.value = Some(v);
                            v
                        }
                    }
                }
            }
        ```

* Unlike functions, closures have access to the variables inside the scope within which the closure is defined.
    ```rust
        fn main() {
            let x: i32 = 4;
            let equal_to_x: |...| -> bool = |z: i32| z == x;
            let y: i32 = 4;
            assert!(equal_to_x(y));
        }
    ```

* * *
## Iterators
* The iterators pattern allows us to iterate over a sequence of items regardless of how they are stored.
    ```rust
        fn main() {
            let v1 = vec![1, 2, 3 ];
            let v1_iter = v1.iter(); // Lazy Iterator
        }
    ```
    * Iterators in Rust are lazy by default until runtime. 
* Iterators are a part of the Rust Standard Library. All iterators implement the `Iterator` trait.
    ```rust
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>; 
        }
    ```
* We can use the map operator to create new iterators.
    ```rust
        fn main(){
            let v1: Vec<i32> = vec![1, 2, 3];
            let v2: Vec<_>  = v1.iter().map(|x: &i32| x+1).collect();
        }
    ```
    * The above code increments the value at iterator `v1` by 1 using the `map()` method, this is then followed by the `collect()` method which collects all the outputs and forms a vector collection called `v2`.

* To take ownership of a collection type we can use a special type of iterator called `into_iter()`. The following example uses a closure input that performs a check on each entry of an iterator.
    ```rust
        fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s: &Shoe| s.size == shoe_size).collect()
        }
    ```
    * The closure defined inside the `filter()` function is able to make use of Shoe size parameters from it's parent environment. 