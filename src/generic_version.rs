

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

pub mod generic_mod {

    use mockall::*;
    use mockall::predicate::*;
    use rand::Rng;
    use std::io::Write;


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

    struct ServoMotor <T:DataInput>{
        // velocity_sensor is the external dependency we like to mock
        // Trait object: Box<dyn...>: Any sensor that implements trait DataInput
        velocity_sensor: T, //Box<dyn DataInput >,
        conversion_factor:i32,
    }

    impl <T:DataInput> ServoMotor <T> {

        fn get_revolution_speed(&self) -> i32 {
            self.velocity_sensor.read_hardware() * self.conversion_factor
        }

        fn new (val:i32, velocity_sensor:T) -> ServoMotor<T> {
        ServoMotor {
            velocity_sensor: velocity_sensor, //Box::new( VelocitySensor{} ),
                conversion_factor: val
            }
        }
    }

    pub fn use_case_manual ()
    {
        let mysensor = VelocitySensor{};
        let motor = ServoMotor {
            velocity_sensor: mysensor, //Box::new( mysensor ),
            conversion_factor:2
        };
        println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
        println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
        println!("Use case a: revolution speed is {}", motor.get_revolution_speed());
    }

    pub fn use_case_with_new ()
    {
        let motor = ServoMotor::new( 2, VelocitySensor{} );
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
                velocity_sensor: mock_data_input,
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
                velocity_sensor: mock_data_input, //Box::new( mock_data_input ),
                conversion_factor:3 };

            // Act: Call the thing
            let result = motor.get_revolution_speed();

            // Assert : Check further assertions
            assert_eq!( result, 33);
        }
    }
}