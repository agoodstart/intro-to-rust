/*
This is an example of bad practise.
Consider you want a part of a string, like the first word.
You would have to break down the string in an array of bytes,
then loop through the array, to find the space,
return the index, and then extract the first word of the string.
This approach is very error prone, and can cause bugs. Rust has a fix for this.
See ownership part five.
*/

pub fn new_main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);

    s.clear();

    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }

    s.len()
}