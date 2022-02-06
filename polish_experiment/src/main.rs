/*
 * https://crates.io/crates/polish
 * https://github.com/Alkass/polish
 */

extern crate polish;

use polish::logger::Logger;
use polish::test_case::{TestCase, TestCaseStatus, TestRunner, Testable};

fn my_test_case(logger: &mut Logger) -> TestCaseStatus {
    // TODO: Your test case code goes here
    TestCaseStatus::PASSED // Other valid statuses are (FAILED, SKIPPED, and UNKNOWN)
}

fn another_test_case(logger: &mut Logger) -> TestCaseStatus {
    TestCaseStatus::PASSED // Other valid statuses are (FAILED, SKIPPED, and UNKNOWN)
}

fn yet_another_test_case(logger: &mut Logger) -> TestCaseStatus {
    TestCaseStatus::PASSED
}

fn test_stuff(logger: &mut Logger) -> TestCaseStatus {
    logger.info(format!("{} + {} = {}", 1, 2, 1 + 2));
    logger.pass(format!(
        "{id}: {message}",
        id = "alkass",
        message = "this is a message"
    ));
    logger.warn(format!("about to fail"));
    logger.fail(format!("failed with err_code: {code}", code = -1));
    TestCaseStatus::PASSED
}

fn test_fail(logger: &mut Logger) -> TestCaseStatus {
    TestCaseStatus::FAILED
}

struct MyTestCase;

impl Testable for MyTestCase {
    fn tests(self) -> Vec<TestCase> {
        vec![
            TestCase::new(
                "Some Title #1",
                "Testing Criteria",
                Box::new(|logger: &mut Logger| -> TestCaseStatus {
                    // TODO: Your test case goes here
                    TestCaseStatus::PASSED
                }),
            ),
            TestCase::new(
                "Some Title #2",
                "Testing Criteria",
                Box::new(|logger: &mut Logger| -> TestCaseStatus {
                    // TODO: Your test case goes here
                    TestCaseStatus::SKIPPED
                }),
            ),

            TestCase::new("","", Box::new(test_fail))
        ]
    }
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

    let mut another_runner = TestRunner::new();
    another_runner.run_test(another_test_case);

    another_runner.run_test(TestCase::new(
        "Title",
        "Crit",
        Box::new(yet_another_test_case),
    ));

    another_runner.run_test(TestCase::new(
        "3rd Test Case Title",
        "Test Case Criteria",
        Box::new(|logger: &mut Logger| -> TestCaseStatus { TestCaseStatus::PASSED }),
    ));

    another_runner.run_test(TestCase::new("", "", Box::new(test_stuff)));

    TestRunner::new().run_tests_from_class(MyTestCase {});
}
