use crate::config::*;
use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::error::Error;
use std::time::{Duration, Instant};

pub struct Sensors {
    pub ultra_trig: OutputPin,
    pub ultra_echo: InputPin,
}

impl Sensors {
    pub fn new(gpio: &Gpio) -> Result<Self, Box<dyn Error>> {
        let ultra_trig = gpio.get(ULTRA_TRIG_PIN)?.into_output();
        let ultra_echo = gpio.get(ULTRA_ECHO_PIN)?.into_input();

        Ok(Sensors {
            ultra_trig,
            ultra_echo,
        })
    }

    pub fn read_distance(&mut self, timeout_ms: u64) -> Result<f64, Box<dyn Error>> {
        let timeout = Duration::from_millis(timeout_ms);

        //Trigger a pulse by setting the trigger pin high for 10 microseconds
        self.ultra_trig.set_low(); //Ensure the trigger pin is low
        std::thread::sleep(Duration::from_micros(2)); //Wait 2 microseconds
        self.ultra_trig.set_high();
        std::thread::sleep(Duration::from_micros(10)); //Pulse for 10 microseconds
        self.ultra_trig.set_low();

        //Wait for the echo pin to go high
        let start_wait = Instant::now();
        while self.ultra_echo.is_low() {
            if start_wait.elapsed() > timeout {
                return Err("Timeout waiting for echo to start".into());
            }
        }

        //Measure the time the echo pin stays high
        let echo_start = Instant::now();
        while self.ultra_echo.is_high() {
            if echo_start.elapsed() > timeout {
                return Err("Timeout waiting for echo to end".into());
            }
        }
        let pulse_duration = echo_start.elapsed();

        //Calculate the distance in cm based on the duration and the speed of sound
        //Speed of sound is ~34300 cm/s, which is ~0.0343 cm per microsecond
        let distance = (pulse_duration.as_micros() as f64) * 0.0343 / 2.0;

        //Return the distance if within a reasonable range, otherwise return an error
        if distance > 400.0 || distance < 2.0 {
            Err("Distance out of range".into())
        } else {
            Ok(distance)
        }
    }
}
