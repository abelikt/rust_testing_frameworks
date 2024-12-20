//!
//! Use-Case: Demo code with hard to predict sensor sensor
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

pub mod legacy_mod {

    use rand::Rng;
    use std::io::Write;

    /// "hard to predict" sensor, a dependency that we like to "cut"
    struct SpeedSensor {}

    /// "hard to predict" functionality of the sensor that we like to "cut"
    impl SpeedSensor {
        fn read_hardware(&self) -> i32 {
            rand::thread_rng().gen_range(0..100)
        }
    }

    struct FanControl {
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

    pub fn use_case_untested_version() {
        let fan = FanControl::new(3);
        println!("Use case untested legacy version: read 10 times revolution speed:");
        for _i in 0..10 {
            print!(" {} ", fan.get_speed());
            std::io::stdout().flush().expect("Flush failed");
        }
        println!();
    }
}
