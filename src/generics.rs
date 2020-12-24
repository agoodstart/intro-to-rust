
pub fn run() {
    let a = func(4, 5);

    println!("{}", a);
}

fn func<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Display>(a: T, b: T) -> T {
    println!("{}", a);
    a - b
}

#[allow(dead_code)]
fn func2<T, E>(a: T, b: T, e: E) -> T where 
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Display,
    E: std::fmt::Display
{
    println!("{}", a);
    println!("{}", e);
    a - b
}

