// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    hello();
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", crate::hello());
}