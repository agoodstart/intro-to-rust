enum IpAddr {
    Ipv4(u8, u8, u8, u8),
    Ipv6(String),
}

pub fn run() {
    let home = IpAddr::Ipv4(172, 168, 10, 1);

    let loopback = IpAddr::Ipv6(String::from("::1"));
}

enum Message {
    Quit,
    Write(String),
    Submit,
    ChangeColor(i32, i32, i32),
}