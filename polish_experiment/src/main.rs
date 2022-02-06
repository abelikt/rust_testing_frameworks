/*
 * https://crates.io/crates/polish
 *
 */

extern crate polish;

use polish::logger::Logger;
use polish::test_case::{TestCase, TestCaseStatus, TestRunner};

fn my_test_case(logger: &mut Logger) -> TestCaseStatus {
    // TODO: Your test case code goes here
    TestCaseStatus::PASSED // Other valid statuses are (FAILED, SKIPPED, and UNKNOWN)
}

fn another_test_case(logger: &mut Logger) -> TestCaseStatus {
    TestCaseStatus::PASSED // Other valid statuses are (FAILED, SKIPPED, and UNKNOWN)
}

fn main() {
    let test_case = TestCase::new(
        "Test Case Title",
        "Test Case Criteria",
        Box::new(my_test_case),
    );
    let another_test_case = TestCase::new(
        "Test Case Title",
        "Test Case Criteria",
        Box::new(another_test_case),
    );
    TestRunner::new().run_test(test_case);
    TestRunner::new().run_test(another_test_case);
}
