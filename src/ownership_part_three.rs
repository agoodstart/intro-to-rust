/*
Although it's possible to transfer strings, it can be tedious when we want to do something with it.
if we want to call a function on a string, we have to return the string everytime we want to do something with it again.
This is where references come in.

References have 2 important rules:
    1.  At any given time, you can have either, but not both of the following:
        one mutable reference or any number of immutable references
    2.  References must always be valid. Better said: references must reference a
        value existing in memory
*/


pub fn _referencing() {
    let s1 = String::from("helllo");
    
    let len = _calculate_length(&s1); // the & means we reference s1, instead of taking ownership

    println!("The string: {}, has a length of: {}", s1, len);
}

fn _calculate_length(s: &String) -> usize { // if we use references as function parameters, it's called borrowing
    s.len() // references are good for doing quick actions
    // references should not live long
    // references can be seen as small pointers, which points to the actual pointer, which points to the memory on the heap
}

pub fn mutable_references() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("s content: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");

    println!("some_string content: {}", some_string);
}

pub fn _dangling_reference() {
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String { // tries to borrow a value, but that value doesn't exist
//     let s = String::from("hello");

//     &s
// }  // s goes out of scope while returning a reference to &s, that's impossible