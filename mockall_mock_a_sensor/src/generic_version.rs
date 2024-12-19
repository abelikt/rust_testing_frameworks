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
//! Create a trait for the function / dependency we like to mock.
//! Make the code that uses the dependency generic with a constraint regarding
//! the trait we like to mock.
//! Use automock to create a mock trait.
//! Inject either the real dependency or the mocked dependency into the code
//! or the tests.
//!
//!
//! Plantuml:
//! [Show in Plantuml](http://www.plantuml.com/plantuml/uml/ZP2zgi9048NxUOfXgtkBl42G40ZOMPa-ZDb1W_iJPYTOYEykIHGCac2txFdEnt3Af2GwU0VGEae9gsZv_JPx2bKCAj7XhiwF75AKMgZJmk0T44tgcMsJfY37sW3YcRMPll_-uG50fwH2hUAi_71C1c6opON4taXuRi2B71qp_ZjvYErrZVxMThvtx3deuguLwmeCus2OPSNcMh0LZksbi-HW1--U)
//!
//!```
//! @startuml
//! class Code
//! class "FanControl <T:SensorTrait>" {
//!   "speed_sensor: T"
//!   get_speed()
//! }
//! abstract class SensorTrait {
//!   read_hardware()
//! }
//! class SpeedSensor {
//!   read_hardware()
//! }
//! class SpeedSensorMock {
//!   read_hardware()
//! }
//! class SpeedSensor implements SensorTrait
//! class SpeedSensorMock implements SensorTrait
//! Code --> "FanControl <T:SensorTrait>"
//! "FanControl <T:SensorTrait>" --> SensorTrait
//! @enduml
//!```

pub mod generic_mod {

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

    struct FanControl<T: SensorTrait> {
        // speed_sensor is the external dependency we like to mock
        speed_sensor: T,
        conversion_factor: i32,
    }

    impl<T: SensorTrait> FanControl<T> {
        fn get_speed(&self) -> i32 {
            self.speed_sensor.read_hardware() * self.conversion_factor
        }
    }

    impl FanControl<SpeedSensor> {
        fn new(val: i32) -> FanControl<SpeedSensor> {
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
                speed_sensor: mock_data_input,
                conversion_factor: 3,
            };

            // Act
            assert_eq!(fan.get_speed(), 30);

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
                speed_sensor: mock_data_input, //Box::new( mock_data_input ),
                conversion_factor: 3,
            };

            // Act: Call the thing
            let result = fan.get_speed();

            // Assert : Check further assertions
            assert_eq!(result, 33);
        }
    }
}
