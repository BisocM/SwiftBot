use crate::config::*;
use crate::utils::clamp;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

pub struct Motors {
    pub motor_en: OutputPin,
    pub motor_left_p: OutputPin,
    pub motor_left_n: OutputPin,
    pub motor_right_p: OutputPin,
    pub motor_right_n: OutputPin,
}

impl Motors {
    pub fn new(gpio: &Gpio) -> Result<Self, Box<dyn Error>> {
        let mut motor_en = gpio.get(MOTOR_EN_PIN)?.into_output();
        motor_en.set_low();

        let mut motor_left_p = gpio.get(MOTOR_LEFT_P)?.into_output();
        let mut motor_left_n = gpio.get(MOTOR_LEFT_N)?.into_output();
        let mut motor_right_p = gpio.get(MOTOR_RIGHT_P)?.into_output();
        let mut motor_right_n = gpio.get(MOTOR_RIGHT_N)?.into_output();

        // Set initial PWM frequency and duty cycle for motor control pins
        for pin in [
            &mut motor_left_p,
            &mut motor_left_n,
            &mut motor_right_p,
            &mut motor_right_n,
        ] {
            pin.set_pwm_frequency(100.0, 0.0)?; // Use set_pwm_frequency
        }

        Ok(Motors {
            motor_en,
            motor_left_p,
            motor_left_n,
            motor_right_p,
            motor_right_n,
        })
    }

    pub fn set_motor_speed(&mut self, motor: u8, speed: f64) -> Result<(), Box<dyn Error>> {
        let speed = clamp(speed, -1.0, 1.0);
        self.motor_en.set_high();

        let (pwm_p, pwm_n) = match motor {
            MOTOR_LEFT => (&mut self.motor_left_p, &mut self.motor_left_n),
            MOTOR_RIGHT => (&mut self.motor_right_p, &mut self.motor_right_n),
            _ => return Err("Invalid motor index".into()),
        };

        let frequency = 100.0;
        let duty_cycle = speed.abs() * 100.0;

        //Correcting motor direction for the left motor
        if motor == MOTOR_LEFT {
            if speed > 0.0 {
                pwm_n.set_pwm_frequency(frequency, duty_cycle)?;
                pwm_p.set_pwm_frequency(frequency, 0.0)?;
            } else {
                pwm_p.set_pwm_frequency(frequency, duty_cycle)?;
                pwm_n.set_pwm_frequency(frequency, 0.0)?;
            }
        } else {
            if speed > 0.0 {
                pwm_p.set_pwm_frequency(frequency, duty_cycle)?;
                pwm_n.set_pwm_frequency(frequency, 0.0)?;
            } else {
                pwm_n.set_pwm_frequency(frequency, duty_cycle)?;
                pwm_p.set_pwm_frequency(frequency, 0.0)?;
            }
        }

        Ok(())
    }

    pub fn disable_motors(&mut self) -> Result<(), Box<dyn Error>> {
        self.motor_en.set_low();
        for pin in [
            &mut self.motor_left_p,
            &mut self.motor_left_n,
            &mut self.motor_right_p,
            &mut self.motor_right_n,
        ] {
            pin.set_pwm_frequency(100.0, 0.0)?;
        }
        Ok(())
    }

    //Motor helper functions
    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_motor_speeds(0.0, 0.0)
    }

    pub fn forward(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.set_motor_speeds(speed, speed)
    }

    pub fn backward(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.set_motor_speeds(-speed, -speed)
    }

    pub fn turn_left(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.set_motor_speeds(-speed, speed)
    }

    pub fn turn_right(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.set_motor_speeds(speed, -speed)
    }

    pub fn set_motor_speeds(
        &mut self,
        left_speed: f64,
        right_speed: f64,
    ) -> Result<(), Box<dyn Error>> {
        self.set_motor_speed(MOTOR_LEFT, left_speed)?;
        self.set_motor_speed(MOTOR_RIGHT, right_speed)
    }

    pub fn set_motor_direction(
        &mut self,
        motor: u8,
        direction: bool,
        speed: f64,
    ) -> Result<(), Box<dyn Error>> {
        let clamped_speed = clamp(speed.abs(), 0.0, 1.0);
        self.motor_en.set_high();

        let (pwm_p, pwm_n) = match motor {
            MOTOR_LEFT => (&mut self.motor_left_p, &mut self.motor_left_n),
            MOTOR_RIGHT => (&mut self.motor_right_p, &mut self.motor_right_n),
            _ => return Err("Invalid motor index".into()),
        };

        let frequency = 100.0;
        let duty_cycle = clamped_speed * 100.0;

        if direction {
            pwm_p.set_pwm_frequency(frequency, duty_cycle)?;
            pwm_n.set_pwm_frequency(frequency, 0.0)?;
        } else {
            pwm_p.set_pwm_frequency(frequency, 0.0)?;
            pwm_n.set_pwm_frequency(frequency, duty_cycle)?;
        }

        Ok(())
    }
}
