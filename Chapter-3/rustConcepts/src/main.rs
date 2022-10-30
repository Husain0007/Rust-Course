fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    let x = "six"; // Shadowing
    println!("The value of x is: {}", x);
    
    const SUBSCRIBER_COUNT: u32 = 100_000; // "const" variables are not
                                          // can add underscores to number literals for readability in Rust

    //***Data Types ***//
    //**Scalar data types : Integer, Floating Point, Boolean, Character.
    //Integer
    let a = 98_222; //Decimal
    let b = 0xff;   //Hex
    let x = 0o77;   //Octal
    let c = 0b1111_0000; //Binary
    let e = b'A'; //Byte(u8 only)

    //Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    //addition
    let sum = 5+10;
    let difference = 95.5 - 4.3;
    let product = 4*30;
    let quotient = 56.7/32.2;
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Character 
    let c = 'z';

    //**Compound data types : 

    // (1)Tuple: fixed size array with related data types
    let tup = ("Husain", 2_500);
    let (name, renum) = tup;
    let renum = tup.1;

    // (2)Arrays: fixed size
    let error_codes = [200, 404, 500];
    let nout_found = error_codes[1];
    let byte = [0;8];

    let sum = my_function(9, 5);
    println!("The sum is: {}", sum);

    // Control Flow : if-else, loops (loop, while, for/for-in)

    let number = 5;

    if number < 10{
        println!("first condition is true");
    } else if number < 22{
        println!("second condition is true");
    } else {
        println!("none of the conditions are true");
    }

    let condition = true;
    let number = if condition {5} else {6};

    //loop
    let mut counter = 0;

    let result = loop{
        counter += 1;
        println!("again!");
        if counter == 10 {
            break counter; // adding "counter" after break returns counter from loop
        }
  
    };
    println!("The result is {}", result);

    let mut number = 3;

    //while loop

    while number !=0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    // for in 

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in 1..4{ // sequence in range excluding last number
        println!("{}!", number);
    }
     
}

fn my_function(x: i32, y: i32) -> i32{
    println!("The value of x is: {}", x); //Statement
    println!("The value of y is: {}", y); //Statement
    //let sum = x+y;// Expression
    //return sum; // Last Expression is implicitly "return", i.e.; we can remove the return keyword
    x + y 
}
