trait SomeCustomTrait {
    fn testest(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct SomeStruct {
    something: i32,
}

impl SomeCustomTrait for SomeStruct {
    fn testest(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeCustomTrait for i32 {
    fn testest(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

pub fn run() {
    let test = SomeStruct { something: 1000 };
    let result = do_this(&test);

    let testi32 = 18;
    let result2 = do_this(&testi32);
}

#[allow(dead_code)]
fn do_this<T>(some_var: &T) -> String
where T: SomeCustomTrait + std::fmt::Debug {
    println!("{:?}", some_var);
    some_var.testest("First", "Second")
}