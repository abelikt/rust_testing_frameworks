//  cargo test -- --nocapture

use rstest::*;

//fun fact this is the second once, it won't be called
#[fixture]
#[once]
fn once_fixture() -> i32 {
    println!("The one and only");
    panic!();
    //0
}

#[fixture]
pub fn my_setup() -> u32 {
    println!("The setup");
    42
}

#[fixture]
pub fn my_other_setup() -> u32 {
    println!("The other setup");
    43
}

struct Context;

impl Drop for Context {
    fn drop(&mut self) {
        println!("Here it comes - The Teardown for context");
    }
}

#[rstest]
fn should_do_stuff(my_setup: u32, my_other_setup: u32) {
    let _c = Context;
    assert_eq!(my_setup, 42);
    assert_eq!(my_other_setup, 43);
}

#[rstest]
fn should_do_stuff_with_explicit_drop(my_setup: u32, my_other_setup: u32) {
    let c = Context;
    assert_eq!(my_setup, 42);
    assert_eq!(my_other_setup, 43);
    drop(c);
}

#[fixture]
pub fn setup_context() -> Context {
    println!("The context setup");
    Context
}

#[rstest]
fn should_do_something_with_context(_setup_context: Context) {
    println!("The context test with setup and teardown");
}
