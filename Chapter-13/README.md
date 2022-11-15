# Chapter 13: Closures

* Closures are anonymous functions. 
* They can be stored as variables and passed around.
* They can also be passed in as input parameters to a function.

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