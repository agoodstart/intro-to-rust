struct User {
    username: String,
    age: u8,
    logged_in: bool
}

pub fn run() {
    let joris = create_user("Joris".to_owned(), 24, true);
}

fn create_user(username: String, age: u8, logged_in: bool) -> User {
    User {
        username,
        age,
        logged_in
    }
}

struct Color {
    red: u8,
    blue: u8,
    yellow: u8
}