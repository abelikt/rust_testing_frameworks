//!
//! Use-Case: Remove dependendy to Velocity Sensor
//!
//!```
//! +----------+       +-----------------+       +-----------------+
//! |          |       |                 |       |                 |
//! |   Stuff  | ----> |   Fan control   | ----> |   Speed Sensor  |
//! |          |       |                 |       |                 |
//! |          |       |   get_speed()   |       | read_hardware() |
//! |          |       |                 |       |                 |
//! +----------+       +-----------------+       +-----------------+
//!```
//!
//! Solution:
//! Create a trait for the function / dependency we like to mock.
//! Make the code that uses the dependency accept a trait object that
//! implements the trait we like to mock.
//! Use automock to create a mock trait.
//! Inject either the real dependency or the mocked dependency into the code
//! or the tests.
//!
//!```
//!    @startuml
//!    class Code
//!    class FanControl
//!    class SpeedSensor implements SpeedSensorTrait
//!    class SpeedSensorMock implements SpeedSensorTrait
//!    Code --> FanControl
//!    FanControl --> SpeedSensorTrait
//!    @enduml
//!```

pub mod trait_object_mod {

    use mockall::predicate::*;
    use mockall::*;
    use rand::Rng;
    use std::io::Write;

    #[automock]
    trait SensorTrait {
        fn read_hardware(&self) -> i32;
    }

    /// "hard to predict" sensor, a dependency that we like to "cut"
    struct SpeedSensor {}

    /// "hard to predict" functionality of the sensor that we like to "cut"
    impl SensorTrait for SpeedSensor {
        fn read_hardware(&self) -> i32 {
            rand::thread_rng().gen_range(0..100)
        }
    }

    struct FanControl {
        // speed_sensor is the external dependency we like to mock
        // Trait object: Box<dyn...>: Any sensor that implements trait SensorTrait
        speed_sensor: Box<dyn SensorTrait>,
        conversion_factor: i32,
    }

    impl FanControl {
        fn get_speed(&self) -> i32 {
            self.speed_sensor.read_hardware() * self.conversion_factor
        }

        fn new(val: i32) -> FanControl {
            FanControl {
                speed_sensor: Box::new(SpeedSensor {}),
                conversion_factor: val,
            }
        }
    }

    pub fn use_case_a_with_inverse_dependency() {
        let mysensor = SpeedSensor {};
        let fan = FanControl {
            speed_sensor: Box::new(mysensor),
            conversion_factor: 2,
        };

        println!("Use case a: speed is: read 10 times speed:");
        for i in 0..10 {
            print!(" {} ", fan.get_speed());
            std::io::stdout().flush().expect("Flush failed");
        }
        println!();
    }

    pub fn use_case_b_with_new() {
        let fan = FanControl::new(2);

        println!("Use case b: speed is: read 10 times speed:");
        for i in 0..10 {
            print!(" {} ", fan.get_speed());
            std::io::stdout().flush().expect("Flush failed");
        }
        println!();
    }

    #[cfg(test)]
    mod test_mod_fan {

        use super::*;

        #[test]
        fn testfan_aaa_test_pattern() {
            // Arrange
            let mut mock_data_input = MockSensorTrait::new();

            mock_data_input
                .expect_read_hardware()
                .with()
                .times(1)
                .returning(|| 10);

            let fan = FanControl {
                speed_sensor: Box::new(mock_data_input),
                conversion_factor: 3,
            };

            // Act
            assert_eq!(fan.get_speed(), 30);

            // Assert
        }

        #[test]
        fn testfan_aaa_pattern_annotated() {
            // Arrange: Create the mock
            let mut mock_data_input = MockSensorTrait::new();

            // Arrange: Configure the mock
            mock_data_input
                .expect_read_hardware()
                .with()
                .times(1)
                .returning(|| 11);

            // Arrange: Crate our thing we like to test
            let fan = FanControl {
                speed_sensor: Box::new(mock_data_input),
                conversion_factor: 3,
            };

            // Act: Call the thing
            let result = fan.get_speed();

            // Assert : Check further assertions
            assert_eq!(result, 33);
        }
    }
}
