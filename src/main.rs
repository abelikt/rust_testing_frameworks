
// https://docs.rs/mockall/0.6.0/mockall/
// It can mock most traits, or structs that only have a single impl block. For things it can't handle, there is mock!.

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

use mockall::*;
use mockall::predicate::*;

use rand::Rng;
//

// cargo test -- --show-output


// https://docs.rs/mockall/0.6.0/mockall/macro.mock.html

// https://docs.rs/crate/mockall_examples/0.7.2/source/src/lib.rs

// more
// https://github.com/kriomant/mockers
// https://www.lpalmieri.com/posts/2020-04-13-wiremock-async-http-mocking-for-rust-applications/

// https://users.rust-lang.org/t/mocking-library/5295
// https://github.com/kriomant/mockers

// https://github.com/asomers/mockall

// mockall

// https://stackoverflow.com/questions/55152927/how-to-mock-specific-methods-but-not-all-of-them-in-rust
// As you have already learned, you cannot replace methods on a type.
// The only thing you can do is move the methods to a trait and then
// provide production and test-specific implementations of that trait.
// How you structure the trait determines the granularity of what you are able to test.

/*********************************************************************/


// https://knowitlabs.no/rust-2020-testing-4ab3d80112ba

/*
 * Within my team it has been debated whether the best style is to
 exclusively depend on abstract interfaces, everywhere. My personal
 view is that this should be determined by the runtime intent. In high
 level languages this is usually no problem, because method dispatch is
 usually dynamic dy default. This means that any or most concrete types
 or classes are easily sub-classable, and thus the coupling to such a
 class by name is not really that strong under the hood. All
 python/Java/Kotlin/JS languages support this easily: Creating mocks of
 concrete types.
 */

/*
Higher order functions

If I want to test a function or procedure A in isolation that depends
on another function B, I can pass B as a parameter to A. In production
code I call A(B). In the test I call A(mocked_B), now with full
control over that dependency.

*/

/*
...
So the way to express an abstraction in Rust is to use a trait.
* Trait mocking in Rust today is fairly developer friendly, but not as
* friendly as in dynamic-dispatch-languages. What I’ve been doing so
* far is to introduce a new trait everywhere I need test isolation,
* and use the crate mockall to autogenerate a mocked implementation
* that I instantiate in my test. Not very far from how java/mockito or
* jest works, except that, as mentioned, we always need to be very
* explicit about the abstraction taking place:

*/

/*
* In Rust, making code dynamic dispatch is less verbose than making
* it static dispatch. This is in some ways counterintuitive, because
* in other areas of the language it appears to be a concious decision
* that less verbose code is simpler and therefore more efficient. Heap
* allocation is one example, it is always very apparent that a heap
* allocation will take place: Box<MyThing> instead of MyThing. A heap
* allocation is more expensive to type and execute. It seems concious
* that there is no short hand for this. First consider dynamic dispatch:
*/


/*********************************************************************/

// Use a trait object Box<dyn DataInput >
// Box<dyn DataInput > is a stand in for any type that implements
// trait DataInput

#[automock]
trait DataInput {
    fn read_hardware(&self) -> i32;
}

struct VelocitySensor {}

impl DataInput for VelocitySensor {
    fn read_hardware(&self) -> i32 {
        rand::thread_rng().gen_range(0,100)
    }
}

struct ServoMotor {
    // VelocitySensor the external dependency we like to mock
    VelocitySensor: Box<dyn DataInput >, // Trait object: Any sensor that implements Readable 3
    conversion_factor:i32,
}

impl ServoMotor {

    fn get_revolution_speed(&self) -> i32 {
        self.VelocitySensor.read_hardware() * self.conversion_factor
    }

    fn new (val:i32) -> ServoMotor {
     ServoMotor {
            VelocitySensor: Box::new( VelocitySensor{} ),
            conversion_factor: val
        }
    }
}


fn use_case_manual ()
{
    let mysensor = VelocitySensor{};
    let motor = ServoMotor {
        VelocitySensor: Box::new( mysensor ),
        conversion_factor:2
    };
    println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
}

fn use_case_with_new ()
{
    let motor = ServoMotor::new( 2 );
    println!("Use case b: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case b: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case b: revolution speed is {}", motor.get_revolution_speed());
}

#[cfg(test)]
mod test_mod_motor {

    use super::*;

    #[test]
    fn def_testmotor_a() {

        // Arrange
        let mut mock_DataInput = MockDataInput::new();

        mock_DataInput.expect_read_hardware()
            .with()
            .times(1)
            .returning( || 45 );

        let motor = ServoMotor{
            VelocitySensor: Box::new( mock_DataInput ),
            conversion_factor:5 };

        // Act
        assert_eq!(motor.get_revolution_speed
    (), 225);

        // Assert
    }

    #[test]
    fn def_testmotor_annotated() {

        // Arrange

        // Create the mock
        let mut mock_DataInput = MockDataInput::new();

        // Configure the mock
        mock_DataInput.expect_read_hardware()
            .with()
            .times(1)
            .returning( || 45 );

        // Crate our DUT
        let motor = ServoMotor{
            VelocitySensor: Box::new( mock_DataInput ),
            conversion_factor:5 };

        // Act: Call DUT

        let result = motor.get_revolution_speed();

        // Assert : Check further assertions

        assert_eq!( result, 225);
    }
}

fn main() {
    use_case_manual();
    use_case_with_new();
}
