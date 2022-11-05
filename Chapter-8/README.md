# Chapter 8 : Common Collections
* While other data structures store only a specific data type at a time, collections can contain multiple data types. Collections are also, unlike built-in data structures like arrays and tuples, stored on the heap.
* Collection types are:
    * Vectors
    * Strings
    * Hashmaps
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