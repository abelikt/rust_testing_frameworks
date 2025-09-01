
# Rust testing frameworks for unit-, integration- and system-tests

... tried out.

And still under heavy but slow development.

Folders herein:

- untested_code: A piece of untested code that should serve as template for
further tests
- mockall_mock_a_sensor: Multiple approaches with cargo test and mockall
(not in sync with the template

- {F}_experiment : Some experiments with Framework F

## Run stuff in the workspace

Build everything

    cargo build --all

Run a specific test

    cargo test --bin mockall_mock_a_sensor

## Used / tested / tried already:

* Cargo test / libtest (built-in) : https://doc.rust-lang.org/cargo/commands/cargo-test.html
* https://crates.io/crates/mockall : A powerful mock object library for Rust.


## ToDo List

Create usability matrix for UT, IT, ST and general impression.


### To-Do list - frameworks:

* https://crates.io/crates/injectorpp : Injectorpp for runtime mocks
* https://crates.io/crates/qutonium : functional testing framework
* https://crates.io/crates/integra8 :  integration test framework
* https://crates.io/crates/git-gamble : blend TCR + TDD
* https://crates.io/crates/rspec :  Write Rspec-like tests with stable rust
* https://crates.io/crates/hamcrest :  A port of the Hamcrest testing library
* https://crates.io/crates/morq :  TDD/BDD assertion interface
* https://crates.io/crates/rs_unit :  RsUnit is a testing crate similar to Elixirs ExUnit
* https://crates.io/crates/stainless :  Organized, flexible testing framework.
* https://crates.io/crates/stainless2 :  Organized, flexible testing framework.
* DONE: https://crates.io/crates/polish :  Test-Driven Development done right
* https://crates.io/crates/speculate ; https://github.com/utkarshkukreti/speculate.rs :
    An RSpec inspired minimal testing framework for Rust.
* DONE: https://crates.io/crates/rstest ; https://github.com/la10736/rstest :
    Rust fixture based test framework. It uses procedural macro to implement
    fixtures and table based tests. (Comparable to pytest fixtures)
* https://crates.io/crates/laboratory ; https://github.com/enokson/laboratory :
    A simple, expressive unit test framework for Rust
* https://crates.io/crates/galvanic-test : Galvanic-test: easier test setup for Rust
* DONE: https://crates.io/crates/cargo-nextest :  A next-generation test runner for Rust.
* https://crates.io/crates/googletest : GoogleTest Rust


### BDD

* https://crates.io/crates/gherkin
* https://crates.io/crates/gherkin_rust
* https://crates.io/crates/cucumber
* https://crates.io/crates/rspec
* https://crates.io/crates/kitten


### To-Do list - helpers, macros, etc

* https://crates.io/crates/galvanic-assert :  set of matcher-based assertions
* https://crates.io/crates/vinegar :  A collection of functions and macros to help testing Rust code
* https://crates.io/crates/test-case : procedural macro attribute for generating parametrized test cases easily
* https://crates.io/crates/serial_test
* https://crates.io/crates/rusty-fork ; https://github.com/altsysrq/rusty-fork :
    Rusty-fork provides a way to "fork" unit tests into separate processes.
* https://crates.io/crates/quickcheck ; https://github.com/BurntSushi/quickcheck :
    Automatic property based testing with shrinking.
* https://crates.io/crates/fake; https://github.com/cksac/fake-rs :  An easy to
    use library for generating fake data like name, number, address, lorem, dates, etc.
* https://crates.io/crates/fluvio-test-derive : Procedure macro to run async fn as test

### To-Do list - test doubles

* HALF: https://crates.io/crates/mockall : A powerful mock object library for Rust.
* https://crates.io/crates/mocktopus :  Mocking framework for Rust
* https://crates.io/crates/shoulda :  derive macro for test assertions
* https://crates.io/crates/simulacrum :  Minimal library for creating mock objects by hand using stable Rust.
* https://crates.io/crates/spy :  Rust spy functions for testing purposes
* https://crates.io/crates/test_double :  Procedural macro for substituting one type for another when testing.
* https://crates.io/crates/galvanic-mock : behaviour-driven mocking for generic traits

#### Run-time mocking

Concepts for installing runtime-mocks, that need no change of the production code.
It is easy e.g. for Python to install runtime mocks to cut out dependencies e.g. with
matching / monkeypatching.

For languages with static type systems, this is almost impossible without
tricks and black magic.
For C++ there is for example Hippomocks (https://github.com/dascandy/hippomocks).
Hippomocks works by changing the op-codes to the functions in the program
memory to the adress of the mock function. A very powerfull trick to my opinion
and a very powerful framework.


Injectorpp seems to work similar to Hippomocks and applies dark and unsafe
powers to do the trick:

https://crates.io/crates/injectorpp

https://docs.rs/injectorpp/0.4.0/injectorpp/

https://github.com/microsoft/injectorppforrust

### More hints are on:

* https://crates.io/search?q=test%20framework
* https://crates.io/search?q=testing%20framework
* https://crates.io/search?q=TDD
* https://en.wikipedia.org/wiki/List_of_unit_testing_frameworks (Sadly, no Rust here yet)
* https://lib.rs/development-tools/testing

## Ideas for new templates

Add a code template for a system test that calls processes;
with other permissions; sudo, sudo -E; runs regex on stderr and stdout;
injects stdin; calls processes in sub-shells; call system management like
systemd.

## Further Links

* https://asomers.github.io/mock_shootout/ Analysis of the author of mockall
* https://nexte.st/ : cargo nextest
* https://www.infinyon.com/blog/2021/04/rust-custom-test-harness/ : How to Build a Custom Test Harness in Rust
* https://github.com/hoodie/awesome-rust-testing

* Helpers e.g. in the tests for cargo: with_stderr_contains
    * https://github.com/rust-lang/cargo/blob/master/crates/cargo-test-support/src/lib.rs
    * https://github.com/rust-lang/cargo/tree/master/crates/cargo-test-support
