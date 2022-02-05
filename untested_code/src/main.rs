/*
*
* This piece of code serves as template code that we want to use to try test-
* frameworks.
* The idea is to copy it into other test folders to see what we can achieve
* with the framework, where we need to modify out template and so on.
*
*
*    +----------+       +-----------------+       +-----------------+
*    |          |       |                 |       |                 |
*    |   Stuff  | ----> |   Fan control   | ----> |   Speed  Sensor |
*    |          |       |                 |       |                 |
*    |          |       |   get_speed()   |       |"read_hardware()"|
*    |          |       |   set_speed()   |       |                 |
*    +----------+       +-----------------+       +-----------------+
*
*/

use rand::Rng;
use std::io::Write;

enum FanError {
    HwFailure,
    CantDo,
}

// This is our "hard to predict" sensor
struct SpeedSensor {}

impl SpeedSensor {
    fn read_hardware(&self) -> Option<i32> {
        // It will respond with some "random" measurement values and also
        // None if reading the speed fails

        let value = rand::thread_rng().gen_range(0, 100);
        if value < 50 {
            Some(value)
        } else {
            None
        }
    }
}

struct FanControl {
    speed_sensor: SpeedSensor,
    conversion_factor: i32,
    speed: i32,
}

impl FanControl {
    fn get_speed(&self) -> i32 {
        match self.speed_sensor.read_hardware() {
            Some(val) => val * self.conversion_factor,
            None => 0,
        }
    }

    fn set_speed(&mut self, speed: i32) -> Result<i32, FanError> {
        if -10 <= speed && speed <= 10 {
            self.speed = speed;
            Ok(speed)
        } else if speed == 100 {
            // This will cause a hardware failure for no real reason
            Err(FanError::HwFailure)
        } else {
            // Here we just complain that we can't do it
            Err(FanError::CantDo)
        }
    }

    fn new(val: i32) -> FanControl {
        FanControl {
            speed_sensor: SpeedSensor {},
            conversion_factor: val,
            speed: 0,
        }
    }
}

pub fn controller() {
    let mut fan = FanControl::new(3);

    println!("Untested legacy version");

    for i in 0..10 {
        // read 10 times revolution speed and print it:
        print!("{i}: {}, ", fan.get_speed());
        std::io::stdout().flush().expect("Flush failed");
    }

    let _result = fan.set_speed(5);

    println!();
}

fn main() {
    controller()
}
