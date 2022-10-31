struct User {
    //attributes
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

fn main() {
    let mut user1 = User {
        email: String::from("syedhusainmustafa@gmail.com"),
        username: String::from("Husain0007"),
        active: true,
        sign_in_count: 12
    };
    let name = user1.username;
    user1.username =  String::from("Mustafa");
}
