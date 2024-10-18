use rppal::i2c::I2c;
use std::error::Error;
use std::fs;

const REG_SHUTDOWN: u8 = 0x00;
const REG_PWM_START: u8 = 0x01;
const REG_ENABLE: u8 = 0x13;
const REG_UPDATE: u8 = 0x16;
const REG_RESET: u8 = 0x17;
const SLAVE_ADDRESS: u16 = 0x54;

pub struct UnderlightLeds {
    pub underlight: [u8; 18], // PWM values for each channel
    pub sn3218: I2c,          // I2C interface for SN3218 communication
}

impl UnderlightLeds {
    /// Initializes the SN3218 LED driver and performs an initial reset.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if !Self::is_i2c_enabled()? {
            return Err("I2C is not enabled on this system. Enable it and try again.".into());
        }

        let mut sn3218 = I2c::new()?;
        sn3218.set_slave_address(SLAVE_ADDRESS)?;

        let underlight = [0u8; 18];

        let mut leds = UnderlightLeds { underlight, sn3218 };
        leds.reset()?;
        leds.enable()?;
        leds.apply_changes()?;

        Ok(leds)
    }

    /// Checks if I2C is enabled on the system by verifying the existence of `/dev/i2c-1`.
    fn is_i2c_enabled() -> Result<bool, Box<dyn Error>> {
        Ok(fs::metadata("/dev/i2c-1").is_ok())
    }

    /// Enables or disables specific channels based on an 18-bit mask.
    pub fn enable_leds(&mut self, enable_mask: u32) -> Result<(), Box<dyn Error>> {
        if enable_mask > 0x3FFFF {
            return Err("Enable mask must be a valid 18-bit value.".into());
        }

        let mask_bytes = [
            (enable_mask & 0x3F) as u8,
            ((enable_mask >> 6) & 0x3F) as u8,
            ((enable_mask >> 12) & 0x3F) as u8,
        ];

        self.sn3218.write(&[REG_ENABLE])?;
        self.sn3218.write(&mask_bytes)?;
        self.apply_changes()
    }

    /// Sets each LED to a specified RGB color (assumes RGB configuration).
    pub fn fill_underlight(&mut self, color: (u8, u8, u8)) -> Result<(), Box<dyn Error>> {
        let (r, g, b) = color;
        for i in 0..6 {
            self.underlight[i * 3] = r;
            self.underlight[i * 3 + 1] = g;
            self.underlight[i * 3 + 2] = b;
        }
        self.write_pwm_values()
    }

    /// Clears all underlighting by setting all channels to zero.
    pub fn clear_underlighting(&mut self) -> Result<(), Box<dyn Error>> {
        self.underlight.fill(0);
        self.update_underlighting()
    }

    /// Sets a specific channel's brightness.
    pub fn set_channel(&mut self, channel: u8, brightness: u8) -> Result<(), Box<dyn Error>> {
        if channel >= 18 {
            return Err(
                format!("Invalid channel ID: {}. Must be between 0 and 17.", channel).into(),
            );
        }
        self.underlight[channel as usize] = brightness;
        Ok(())
    }

    /// Sets an RGB LED's color (for RGB configuration).
    pub fn set_rgb(&mut self, led_id: u8, r: u8, g: u8, b: u8) -> Result<(), Box<dyn Error>> {
        if led_id >= 6 {
            return Err(format!("Invalid RGB LED ID: {}. Must be between 0 and 5.", led_id).into());
        }
        let base_channel = led_id as usize * 3;
        self.underlight[base_channel] = r;
        self.underlight[base_channel + 1] = g;
        self.underlight[base_channel + 2] = b;
        Ok(())
    }

    /// Updates the SN3218 to reflect the current values in `underlight`.
    pub fn update_underlighting(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_pwm_values()
    }

    /// Fills all channels with the same brightness level.
    pub fn fill_all(&mut self, brightness: u8) -> Result<(), Box<dyn Error>> {
        self.underlight.fill(brightness);
        self.update_underlighting()
    }

    /// Resets the SN3218 device.
    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.sn3218
            .write(&[REG_RESET, 0xFF])
            .map_err(|_| Box::<dyn Error>::from("Failed to reset SN3218."))?;
        Ok(())
    }

    /// Enables SN3218 output.
    pub fn enable(&mut self) -> Result<(), Box<dyn Error>> {
        self.sn3218
            .write(&[REG_ENABLE, 0x00])
            .map_err(|_| Box::<dyn Error>::from("Failed to enable SN3218."))?;
        Ok(())
    }

    /// Disables SN3218 output.
    pub fn disable(&mut self) -> Result<(), Box<dyn Error>> {
        self.sn3218
            .write(&[REG_ENABLE, 0x01])
            .map_err(|_| Box::<dyn Error>::from("Failed to disable SN3218."))?;
        Ok(())
    }

    /// Applies the current PWM values and settings to the SN3218.
    fn apply_changes(&mut self) -> Result<(), Box<dyn Error>> {
        self.sn3218
            .write(&[REG_UPDATE, 0xFF])
            .map_err(|_| Box::<dyn Error>::from("Failed to apply changes to SN3218."))?;
        Ok(())
    }

    /// Writes the `underlight` buffer to the PWM registers.
    fn write_pwm_values(&mut self) -> Result<(), Box<dyn Error>> {
        self.sn3218
            .write(&[REG_PWM_START])
            .map_err(|_| Box::<dyn Error>::from("Failed to start register writing for PWM."))?;
        self.sn3218
            .write(&self.underlight)
            .map_err(|_| Box::<dyn Error>::from("Failed to write PWM values to SN3218."))?;
        self.apply_changes()?;
        Ok(())
    }
}
