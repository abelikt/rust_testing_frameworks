// Links:
// https://docs.rs/mockall/0.6.0/mockall/
// It can mock most traits, or structs that only have a single impl block.
// For things it can't handle, there is mock!.

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod generic_version;
mod generic_version_no_struct;
mod legacy_version;
mod trait_object_version;

use generic_version::generic_mod;
use generic_version_no_struct::generic_no_struct_mod;
use legacy_version::legacy_mod;
use trait_object_version::trait_object_mod;

// cargo run
// cargo test -- --show-output
// cargo run && cargo test
// cargo run && cargo test -- --show-output

/*
*    Use-Case: Test a fan controller that depends on a non deterministic sensor
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Fan control   | ----> |   Speed  Sensor |
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
*    |   Stuff  | ----> |   Fan control   | ----> |       Mock      |
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

    generic_no_struct_mod::use_case_a_with_inverse_dependeny();
    generic_no_struct_mod::use_case_b_with_new();

    println!();
}
