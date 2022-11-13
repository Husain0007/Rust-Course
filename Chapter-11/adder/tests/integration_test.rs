use adder; // bringing library into scope
mod common;
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, adder::add_two(2)); 
}