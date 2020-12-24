pub fn a_string() {
    let s = String::from("hello");

    let bytes = str_to_bytes(&s);

    println!("{:?}", bytes);
}

fn str_to_bytes(s: &String) -> &[u8] {

    s.as_bytes()

}