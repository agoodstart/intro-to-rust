/*
String slices fix the problem we have with bad_practise.
A string slice is a reference to a part of a string.
*/

pub fn _slice() {
    let s = String::from("Hello, world!");

    let hello = &s[0..6]; // if you know ruby, this looks similar, it kind of acts similar as well
    let world = &s[7..13];  // range starts with a starting index, and end with a ending index

    println!("{} {}", hello, world);
}

/* 
String slices are references, but string literals get neither stored on the heap nor the stack!
It gets stored in your programs' binary. The pointer points to the place in the binary.
*/

// fixing the bad practise
pub fn new_main() {
    let s = String::from("Hello world");

    let word = first_word(&s[..]);

    // s.clear(); // this wouldn't work anymore

    println!("{}", word);

    let sl = "this is a string literal";

    // since string literals are references, you don't have to use & in the function call:
    let new_word = first_word(sl);
    println!("{}", new_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i]; // change this so it returns a slice
        }
    }

    &s[..] 
}

