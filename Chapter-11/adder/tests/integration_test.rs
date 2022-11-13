use adder; // bringing library into scope

#[test]
fn it_adds_two(){
    assert_eq!(4, adder::add_two(2)); 
}