pub fn run() {
    let mut s = String::from("hello");
    let ptr = s.as_ptr();
    let l = s.len();
    let c  = s.capacity();

    println!("String s contains {}, with pointer: {:?}, Length: {}, Capacity: {}", s, ptr, l, c);

    let news = ", world!";

    s.push_str(&*news);

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 pointer: {:?}, s2 pointer: {:?}", s1.as_ptr(), s2.as_ptr());

    let mut s3 = String::from("This string");
    let mut _s4 = change(&mut s3);

    println!("{}", s3);
}

fn change(some_string: &mut String) {
    some_string.push_str(" will change.");
}