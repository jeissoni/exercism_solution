use hello_world::hello;

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, World!");
}
