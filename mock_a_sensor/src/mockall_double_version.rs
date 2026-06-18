//!
//! Use-Case: Remove dependendy to Velocity Sensor
//!
//!```
//!    +----------+       +-----------------+       +-----------------+
//!    |          |       |                 |       |                 |
//!    |   Stuff  | ----> |   Fan control   | ----> |   Speed  Sensor |
//!    |          |       |                 |       |                 |
//!    |          |       |   get_speed()   |       | read_hardware() |
//!    |          |       |                 |       |                 |
//!    +----------+       +-----------------+       +-----------------+
//!```
//!
//! Solution:
//!
//! Use a compile switch to inject the test double. Can also be applied for
//! use statements on functions e.g. from std.
//!
//!

pub mod mockall_double_mod {

    use std::io::Write;

    pub mod sensor_mod {
        /// "hard to predict" sensor, a dependency that we like to "cut"
        pub struct SpeedSensor {}

        #[cfg(test)]
        pub struct MockSpeedSensor {}

        impl SpeedSensor {
            pub fn read_hardware(&self) -> i32 {
                rand::random_range(0..100)
            }
        }

        #[cfg(test)]
        impl MockSpeedSensor {
            pub fn read_hardware(&self) -> i32 {
                110
            }
        }
    }

    use sensor_mod::SpeedSensor;

    struct FanControl {
        // speed_sensor is the external dependency we like to mock
        speed_sensor: SpeedSensor,
        conversion_factor: i32,
    }

    impl FanControl {
        fn get_speed(&self) -> i32 {
            self.speed_sensor.read_hardware() * self.conversion_factor
        }

        fn new(val: i32) -> FanControl {
            FanControl {
                speed_sensor: SpeedSensor {},
                conversion_factor: val,
            }
        }
    }

    pub fn use_case_a_with_inverse_dependency() {
        let mysensor = SpeedSensor {};
        let fan = FanControl {
            speed_sensor: mysensor,
            conversion_factor: 2,
        };

        println!("Use case a: speed is: read 10 times speed:");
        for _i in 0..10 {
            print!(" {} ", fan.get_speed());
            std::io::stdout().flush().expect("Flush failed");
        }
        println!();
    }

    pub fn use_case_b_with_new() {
        let fan = FanControl::new(2);
        println!("Use case b: speed is: read 10 times speed:");
        for _i in 0..10 {
            print!(" {} ", fan.get_speed());
            std::io::stdout().flush().expect("Flush failed");
        }
    }

    #[cfg(test)]
    mod test_mod_fan {

        use crate::mockall_double_mod as md;
        use mockall_double::double;

        #[double]
        use crate::mockall_double_mod::sensor_mod::SpeedSensor;

        #[test]
        fn testfancontrol_aaa_test_pattern() {
            // Arrange

            let fan = md::FanControl {
                // Error: expected `SpeedSensor`, found `MockSpeedSensor`
                speed_sensor: SpeedSensor {},
                conversion_factor: 3,
            };

            // Act
            assert_eq!(fan.get_speed(), 30);

            // Assert
        }
    }
}
