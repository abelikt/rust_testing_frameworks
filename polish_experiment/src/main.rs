
/*
 * https://crates.io/crates/polish
 *
 */

extern crate polish;

use polish::test_case::{TestRunner, TestCaseStatus, TestCase};
use polish::logger::Logger;

fn my_test_case(logger: &mut Logger) -> TestCaseStatus {
  // TODO: Your test case code goes here
  TestCaseStatus::PASSED // Other valid statuses are (FAILED, SKIPPED, and UNKNOWN)
}

fn main() {
  let test_case = TestCase::new("Test Case Title", "Test Case Criteria", Box::new(my_test_case));
  TestRunner::new().run_test(test_case);
}
