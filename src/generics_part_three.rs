#[allow(dead_code)]
struct SomeStruct<T, U>{
    some_t: T,
    some_u: U,
}

impl<T, U> SomeStruct<T, U> where  
T: std::fmt::Debug,
U: std::fmt::Debug
{
    fn log_something(&self) {
        println!("{:?} {:?}", self.some_t, self.some_u);
    }
}

pub fn run() {
    let test = SomeStruct {
        some_t: 5.6,
        some_u: vec![1, 2, 3],
    };

    test.log_something();
}