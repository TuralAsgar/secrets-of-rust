pub fn print() {
    println!("Hello, World!");
}

pub fn world() -> String {
    String::from("Hello, World")
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_2_and_2_returns_4() {
    let result = add(2, 2);
    assert_eq!(result, 4, "add(2, 2): want 4, got {result}");
}

#[test]
fn always_passes() {}

#[test]
fn always_fails() {
    panic!("Oh no!");
}

#[test]
fn world_returns_hello_world() {
    assert_eq!(world(), "Hello, World");
}
