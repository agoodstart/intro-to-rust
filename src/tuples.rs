
pub fn tuple() {
    let t: (char, i32, bool) = ('c', 54, true);

    println!("A tuple: {:?}", t);

    let u = t.2;

    println!("extracted from the tuple: {}", u);

    let (a, _, _) = t;

    println!("Also extracted from the tuple: {}", a);
}