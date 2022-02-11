
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
