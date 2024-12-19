//!
//! Use-Case: Remove dependendy to Velocity Sensor
//!
//!```
//!    +----------+       +-----------------+       +-----------------+
//!    |          |       |                 |       |                 |
//!    |   Stuff  | ----> |   Fan control   | ----> |  Speed  Sensor  |
//!    |          |       |                 |       |   (no struct)   |
//!    |          |       |   get_speed()   |       | read_hardware() |
//!    |          |       |                 |       |                 |
//!    +----------+       +-----------------+       +-----------------+
//!```
//!
//! Solution:
//!
//! This is a similar approach where we have no struct for the sensor
//! but a SensorTrait in the background. The dependency is injected via
//! the call to get speed, which is generic.
//! I found a tip to a similar solution on reddit but after implementing it
//! I'm not convinced that it helps a lot.

pub mod generic_no_struct_mod {

    use mockall::predicate::*;
    use mockall::*;
    use rand::Rng;
    use std::io::Write;

    struct SpeedSensor;

    #[automock]
    trait SensorTrait {
        fn read_hardware(&self) -> i32;
    }

    /// "hard to predict" functionality of the sensor that we like to "cut"
    impl SensorTrait for SpeedSensor{
        fn read_hardware(&self) -> i32 {
            rand::thread_rng().gen_range(0..100)
        }
    }

    struct FanControl {
        conversion_factor: i32,
    }

    impl FanControl {
        fn get_speed<T:SensorTrait>(&self, t:&T) -> i32 {
            t.read_hardware() * self.conversion_factor
        }
    }

    impl FanControl{
        fn new(val: i32) -> FanControl{
            FanControl {
                conversion_factor: val,
            }
        }
    }

    pub fn use_case_a_with_inverse_dependeny() {
        let fan = FanControl {
            conversion_factor: 2,
        };
        let a = SpeedSensor;
        println!("Use case a: speed is: read 10 times speed:");
        for i in 0..10 {
            print!(" {} ", fan.get_speed(&a));
            std::io::stdout().flush().expect("Flush failed");
        }
        println!();
    }

    pub fn use_case_b_with_new() {
        let fan = FanControl::new(2);
        println!("Use case b: speed is: read 10 times speed:");
        let a = SpeedSensor;
        for i in 0..10 {
            print!(" {} ", fan.get_speed(&a));
            std::io::stdout().flush().expect("Flush failed");
        }
    }

    #[cfg(test)]
    mod test_mod_fan {

        use super::*;

        #[test]
        fn testfancontrol_aaa_test_pattern() {
            // Arrange
            let mut mock_data_input = MockSensorTrait::new();

            mock_data_input
                .expect_read_hardware()
                .with()
                .times(1)
                .returning(|| 10);

            let fan = FanControl {
                conversion_factor: 3,
            };

            // Act
            assert_eq!(fan.get_speed(&mock_data_input), 30);

            // Assert
        }

        #[test]
        fn testfancontrol_aaa_pattern_annotated() {
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
                conversion_factor: 3,
            };

            // Act: Call the thing
            let result = fan.get_speed(&mock_data_input);

            // Assert : Check further assertions
            assert_eq!(result, 33);
        }
    }
}
