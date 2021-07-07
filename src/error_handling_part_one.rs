use std::fs::File;

// The Result type is an enum, constructed as follows
enum Result<T, E>{
    Ok(T),
    Err(E),
}

pub fn run() {
    panic!("crash and burn");
}

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem with the file {}", error);
        },
    };
}