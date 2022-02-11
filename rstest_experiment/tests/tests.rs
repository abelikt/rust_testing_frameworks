

/* Won't work as long we only have a binary crate

//use super::*;
//use rstest_experiment;
use rstest::*;

#[rstest]
fn another_single_somewhere_else(once_fixture: &i32) {
    // All tests that use once_fixture will share the same reference to once_fixture() 
    // function result.
    assert_eq!(&42, once_fixture)
}

*/
