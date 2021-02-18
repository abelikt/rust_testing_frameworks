

// Links:
// https://docs.rs/mockall/0.6.0/mockall/
// It can mock most traits, or structs that only have a single impl block.
// For things it can't handle, there is mock!.

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

use mockall::*;
use mockall::predicate::*;
use rand::Rng;

// cargo test -- --show-output

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

/*********************************************************************/
/*
*    Use-Case: Remove dependendy to Velocity Sensor
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Servo Motor   | ----> | Velocity Sensor |
*    |          |       |                 |       |                 |
*    |          |       |   get_speed()   |       | read_hardware() |
*    |          |       |                 |       |                 |
*    +----------+       +-----------------+       +-----------------+
*
*/


// This is our "hard to predict" sensor
struct VelocitySensorLegacy {}

impl VelocitySensorLegacy {
    fn read_hardware(&self) -> i32 {
        rand::thread_rng().gen_range(0, 100)
    }
}

struct ServoMotorLegacy {
    velocity_sensor_legacy: VelocitySensorLegacy,
    conversion_factor:i32,
}

impl ServoMotorLegacy {

    fn get_revolution_speed(&self) -> i32 {
        self.velocity_sensor_legacy.read_hardware() * self.conversion_factor
    }

    fn new (val:i32) -> ServoMotorLegacy {
     ServoMotorLegacy {
            velocity_sensor_legacy : VelocitySensorLegacy{},
            conversion_factor: val
        }
    }
}

/*********************************************************************/
/*
*    Use-Case: Remove dependendy to Velocity Sensor
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Servo Motor   | ----> | Velocity Sensor |
*    |          |       |                 |       |                 |
*    |          |       |   get_speed()   |       | read_hardware() |
*    |          |       |                 |       |                 |
*    +----------+       +-----------------+       +-----------------+
*
*/

#[automock]
trait DataInput {
    fn read_hardware(&self) -> i32;
}

// This is our "hard to predict" sensor, dependency we like to
// remove
struct VelocitySensor {}

impl DataInput for VelocitySensor {
    fn read_hardware(&self) -> i32 {
        rand::thread_rng().gen_range(0,100)
    }
}

struct ServoMotor {
    // velocity_sensor is the external dependency we like to mock
    // Trait object: Box<dyn...>: Any sensor that implements trait DataInput
    velocity_sensor: Box<dyn DataInput >,
    conversion_factor:i32,
}

impl ServoMotor {

    fn get_revolution_speed(&self) -> i32 {
        self.velocity_sensor.read_hardware() * self.conversion_factor
    }

    fn new (val:i32) -> ServoMotor {
     ServoMotor {
        velocity_sensor: Box::new( VelocitySensor{} ),
            conversion_factor: val
        }
    }
}

fn use_case_untested_version ()
{
    let motor = ServoMotorLegacy::new(3);

    println!("Use case untested: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case untested: revolution speed is {}", motor.get_revolution_speed());
    println!("Use case untested: revolution speed is {}", motor.get_revolution_speed());
}

fn use_case_manual ()
{
    let mysensor = VelocitySensor{};
    let motor = ServoMotor {
        velocity_sensor: Box::new( mysensor ),
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
    fn testmotor_aaa_test_pattern() {

        // Arrange
        let mut mock_data_input = MockDataInput::new();

        mock_data_input.expect_read_hardware()
            .with()
            .times(1)
            .returning( || 10 );

        let motor = ServoMotor{
            velocity_sensor: Box::new( mock_data_input ),
            conversion_factor:3 };

        // Act
        assert_eq!(motor.get_revolution_speed(), 30);

        // Assert
    }

    #[test]
    fn testmotor_aaa_pattern_annotated() {
        // Arrange: Create the mock
        let mut mock_data_input = MockDataInput::new();

        // Arrange: Configure the mock
        mock_data_input.expect_read_hardware()
            .with()
            .times(1)
            .returning( || 11 );

        // Arrange: Crate our thing we like to test
        let motor = ServoMotor{
            velocity_sensor: Box::new( mock_data_input ),
            conversion_factor:3 };

        // Act: Call the thing
        let result = motor.get_revolution_speed();

        // Assert : Check further assertions
        assert_eq!( result, 33);
    }
}

fn main() {
    use_case_manual();
    use_case_with_new();
}
