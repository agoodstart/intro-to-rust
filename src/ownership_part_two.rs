/*
function will have a different scope, which may impact your code
*/

pub fn _functions() {
    let s = String::from("hello"); // s comes into scope
    _takes_ownership(s); // s moves into this function and is no longer valid after

    let x = 5; // x comes into scope
    _makes_copy(x); // x is still valid after this function call, because i32 has Copy

    println!("{}", x);
    // println!("{}", s);


}

fn _takes_ownership(_some_string: String) { }

fn _makes_copy(_num: i32) { }

pub fn transferring_values() {
    let s1 = gives_ownership(); // calls function, and moves the result into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // takes s2, moves it into the function, the function then return the string and moves it into s3

    println!("{}", s1);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("gives ownership to s1");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}