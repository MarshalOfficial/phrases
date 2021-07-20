
pub fn hello() -> String {
    "hello".to_string()
}
pub fn goodby() -> String {
    "goodby".to_string()
}

#[test]
// #[should_panic]
// #[ignore]
fn english_greeting_correct(){
    assert_eq!("hello",hello());
}