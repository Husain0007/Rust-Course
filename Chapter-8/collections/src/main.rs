fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new(); // dynamically allocated so we have to specify type

    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1, 2, 3]; // Using vec! macros to initialize vector with values
    } // value of v2 is dropped outside scope

    println!("{}",v[0]);

}
