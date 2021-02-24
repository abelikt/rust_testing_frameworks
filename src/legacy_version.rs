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

pub mod legacy_mod{

    use rand::Rng;

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

    pub fn use_case_untested_version ()
    {
        let motor = ServoMotorLegacy::new(3);

        println!("Use case untested Legacy: revolution speed is {}", motor.get_revolution_speed());
        println!("Use case untested Legacy: revolution speed is {}", motor.get_revolution_speed());
        println!("Use case untested Legacy: revolution speed is {}", motor.get_revolution_speed());
    }

}
