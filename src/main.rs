
// Links:
// https://docs.rs/mockall/0.6.0/mockall/
// It can mock most traits, or structs that only have a single impl block.
// For things it can't handle, there is mock!.

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod legacy_version;
mod generic_version;
mod trait_object_version;

use legacy_version::legacy_mod;
use generic_version::generic_mod;
use trait_object_version::trait_object_mod; 

// cargo run
// cargo test -- --show-output
// cargo run && cargo test
// cargo run && cargo test -- --show-output

// Related:

// https://stackoverflow.com/questions/55152927/how-to-mock-specific-methods-but-not-all-of-them-in-rust
// "As you have already learned, you cannot replace methods on a type.
// The only thing you can do is move the methods to a trait and then
// provide production and test-specific implementations of that trait.
// How you structure the trait determines the granularity of what you
// are able to test.

// https://knowitlabs.no/rust-2020-testing-4ab3d80112ba

/*
* Within my team it has been debated whether the best style is to
* exclusively depend on abstract interfaces, everywhere. My personal
* view is that this should be determined by the runtime intent. In high
* level languages this is usually no problem, because method dispatch is
* usually dynamic dy default. This means that any or most concrete types
* or classes are easily sub-classable, and thus the coupling to such a
* class by name is not really that strong under the hood. All
* python/Java/Kotlin/JS languages support this easily: Creating mocks of
* concrete types.
* ...
* So the way to express an abstraction in Rust is to use a trait.
* Trait mocking in Rust today is fairly developer friendly, but not as
* friendly as in dynamic-dispatch-languages. What I’ve been doing so
* far is to introduce a new trait everywhere I need test isolation,
* and use the crate mockall to autogenerate a mocked implementation
* that I instantiate in my test. Not very far from how java/mockito or
* jest works, except that, as mentioned, we always need to be very
* explicit about the abstraction taking place:
*/




/*
*    Use-Case: Test a servo motor that depends on a non deterministic sensor
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Servo Motor   | ----> | Velocity Sensor |
*    |          |       |                 |       |                 |
*    |          |       |   get_speed()   |       | read_hardware() |
*    |          |       |                 |       |                 |
*    +----------+       +-----------------+       +-----------------+
*
*
*    Objective: For tests: remove dependendy to the sensor
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Servo Motor   | ----> |       Mock      |
*    |          |       |                 |       |                 |
*    |          |       |   get_speed()   |       | read_hardware() |
*    |          |       |                 |       |                 |
*    +----------+       +-----------------+       +-----------------+
*
*
* Options:
* 
*  - Use a sensor-trait and trait objects
*  - Use a sensor-trait and generics
* 
*/


fn main() {
    println!();

    legacy_mod::use_case_untested_version();

    trait_object_mod::use_case_a_with_inverse_dependeny();
    trait_object_mod::use_case_b_with_new();

    generic_mod::use_case_a_with_inverse_dependeny();
    generic_mod::use_case_b_with_new();

    println!();
}
