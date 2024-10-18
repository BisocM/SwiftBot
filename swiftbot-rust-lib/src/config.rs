//Button IDs
pub const BUTTON_A: u8 = 0;
pub const BUTTON_B: u8 = 1;
pub const BUTTON_X: u8 = 2;
pub const BUTTON_Y: u8 = 3;
pub const NUM_BUTTONS: usize = 4;

//Underlighting LED locations
pub const LIGHT_FRONT_RIGHT: u8 = 0;
pub const LIGHT_FRONT_LEFT: u8 = 1;
pub const LIGHT_MIDDLE_LEFT: u8 = 2;
pub const LIGHT_REAR_LEFT: u8 = 3;
pub const LIGHT_REAR_RIGHT: u8 = 4;
pub const LIGHT_MIDDLE_RIGHT: u8 = 5;
pub const NUM_UNDERLIGHTS: usize = 6;

//Motor names
pub const MOTOR_LEFT: u8 = 0;
pub const MOTOR_RIGHT: u8 = 1;
pub const NUM_MOTORS: usize = 2;

//GPIO pins
pub const BUTTON_A_PIN: u8 = 5;
pub const BUTTON_B_PIN: u8 = 6;
pub const BUTTON_X_PIN: u8 = 16;
pub const BUTTON_Y_PIN: u8 = 24;

//Onboard LEDs pins (next to each button)
pub const LED_A_PIN: u8 = 23;
pub const LED_B_PIN: u8 = 22;
pub const LED_X_PIN: u8 = 17;
pub const LED_Y_PIN: u8 = 27;

//Motor driver pins
pub const MOTOR_EN_PIN: u8 = 26;
pub const MOTOR_LEFT_P: u8 = 8;
pub const MOTOR_LEFT_N: u8 = 11;
pub const MOTOR_RIGHT_P: u8 = 10;
pub const MOTOR_RIGHT_N: u8 = 9;

//HC-SR04 Ultrasound pins
pub const ULTRA_TRIG_PIN: u8 = 13;
pub const ULTRA_ECHO_PIN: u8 = 25;

//Speed of sound in cm/ns
pub const SPEED_OF_SOUND_CM_NS: f64 = 343.0 * 100.0 / 1e9;
