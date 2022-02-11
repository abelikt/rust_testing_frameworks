
/*
 *
 *
 * https://crates.io/crates/rstest
 *
 * cargo test -- --nocapture
 *
 *
 */


fn main() {
    println!("Hello, world!");
}

use rstest::*;

#[fixture]
pub fn fixture() -> u32 { 42 }

#[rstest]
fn should_success(fixture: u32) {
    assert_eq!(fixture, 42);
}

#[rstest]
#[should_panic]
fn should_fail(fixture: u32) {
    assert_ne!(fixture, 42);
}



#[fixture]
#[once]
fn once_fixture() -> i32 {
    println!("Only once");
    42 }

#[rstest]
fn single(once_fixture: &i32) {
    // All tests that use once_fixture will share the same reference to once_fixture() 
    // function result.
    assert_eq!(&42, once_fixture)
}

#[rstest]
fn second_single(once_fixture: &i32) {
    // All tests that use once_fixture will share the same reference to once_fixture() 
    // function result.
    assert_eq!(&42, once_fixture)
}



// Fixtures that call other fixtures seem to be unused for LSP


struct User {name: String, age: u8}

impl User {

    fn new(name:&str, age:u8) -> User {
        User{name:name.to_string(), age}
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u8 {
        self.age
    }
}


#[fixture]
fn user(#[default("Alice")] name: &str, #[default(22)] age: u8) -> User {
    User::new(name, age)
}

#[rstest]
fn is_alice(user: User) {
    assert_eq!(user.name(), "Alice")
}

#[rstest]
fn is_22(user: User) {
    assert_eq!(user.age(), 22)
}

#[rstest]
fn is_bob(#[with("Bob")] user: User) {
    assert_eq!(user.name(), "Bob")
}

#[rstest]
fn is_42(#[with("", 42)] user: User) {
    assert_eq!(user.age(), 42)
}



