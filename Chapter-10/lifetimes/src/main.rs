struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a>{
    fn return_part (&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let result;

    
    let string2 = String::from("xyz");
    // Using Lifetimes the string with the shortest Lifetime is returned
    // In this case it will be string2
    result = longest(string1, string2);
    
    println!("The longest string is {}", result.as_str());

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
// &i32 : a reference
//&'a i32 : a reference with an explicit lifetime
//&'a mut i32 : a mutable reference with an explicit lifeti me

fn longest<'a>(x:String, y:String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
