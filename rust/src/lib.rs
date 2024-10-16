#[macro_use]
extern crate lazy_static;
mod config;
mod buttons;
mod motors;
mod sensors;
mod utils;
mod camera;
mod sn3218;

use crate::sn3218::UnderlightLeds;
use crate::motors::Motors;
use crate::sensors::Sensors;
use crate::buttons::Buttons;
use crate::camera::CameraController;
use crate::config::{BUTTON_A_PIN, BUTTON_B_PIN, BUTTON_X_PIN, BUTTON_Y_PIN};

use jni::objects::JClass;
use jni::sys::{jboolean, jbyteArray, jdouble, jint};
use jni::JNIEnv;
use std::sync::Mutex;
use rppal::gpio::{Gpio, InputPin};

//Use lazy_static to create static instances accessible across JNI calls
lazy_static! {
    /// Global GPIO instance used throughout the application.
    static ref GPIO: Gpio = Gpio::new().unwrap();

    /// Shared instance of the `Leds` struct,protected by a `Mutex` for thread safety.
    /// Used for control of the LEDs at the top of the robot.
    static ref BUTTONLEDS: Mutex<Buttons> = Mutex::new(Buttons::new(&GPIO).unwrap());

    /// Shared instance of the `UnderlightLeds` struct, protected by a `Mutex` for thread safety.
    /// Used for control of the SN3218 underlight LEDs.
    static ref SN3218LEDS: Mutex<UnderlightLeds> = Mutex::new(UnderlightLeds::new().unwrap());

    /// Shared instance of the `Motors` struct, protected by a `Mutex` for thread safety.
    static ref MOTORS: Mutex<Motors> = Mutex::new(Motors::new(&GPIO).unwrap());

    /// Shared instance of the `Sensors` struct, protected by a `Mutex` for thread safety.
    static ref SENSORS: Mutex<Sensors> = Mutex::new(Sensors::new(&GPIO).unwrap());

    /// Shared instance of the `CameraController` struct, protected by a `Mutex` for thread safety.
    static ref CAMERA: Mutex<CameraController> = Mutex::new(CameraController::new().unwrap());

    //Button pins
    static ref BUTTON_A_INPUT: InputPin = GPIO.get(BUTTON_A_PIN).unwrap().into_input_pulldown();
    static ref BUTTON_B_INPUT: InputPin = GPIO.get(BUTTON_B_PIN).unwrap().into_input_pulldown();
    static ref BUTTON_X_INPUT: InputPin = GPIO.get(BUTTON_X_PIN).unwrap().into_input_pulldown();
    static ref BUTTON_Y_INPUT: InputPin = GPIO.get(BUTTON_Y_PIN).unwrap().into_input_pulldown();
}

/// Stops the robot by setting motor speeds to zero.
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error stopping the motors.
///
/// # JNI Signature
///
/// ```java
/// public static native void stop();
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_stop(
    mut env: JNIEnv,
    _class: JClass,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.stop() {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Moves the robot forward at the specified speed.
///
/// # Arguments
///
/// * `speed` - The speed at which to move forward (0.0 to 1.0).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the motor speeds.
///
/// # JNI Signature
///
/// ```java
/// public static native void forward(double speed);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_forward(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.forward(speed as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Moves the robot backward at the specified speed.
///
/// # Arguments
///
/// * `speed` - The speed at which to move backward (0.0 to 1.0).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the motor speeds.
///
/// # JNI Signature
///
/// ```java
/// public static native void backward(double speed);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_backward(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.backward(speed as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Turns the robot to the left at the specified speed.
///
/// # Arguments
///
/// * `speed` - The speed at which to turn left (0.0 to 1.0).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the motor speeds.
///
/// # JNI Signature
///
/// ```java
/// public static native void turnLeft(double speed);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_turnLeft(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.turn_left(speed as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Turns the robot to the right at the specified speed.
///
/// # Arguments
///
/// * `speed` - The speed at which to turn right (0.0 to 1.0).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the motor speeds.
///
/// # JNI Signature
///
/// ```java
/// public static native void turnRight(double speed);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_turnRight(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.turn_right(speed as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Sets the speeds of the left and right motors individually.
///
/// # Arguments
///
/// * `left_speed` - Speed of the left motor (-1.0 to 1.0).
/// * `right_speed` - Speed of the right motor (-1.0 to 1.0).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the motor speeds.
///
/// # JNI Signature
///
/// ```java
/// public static native void setMotorSpeeds(double leftSpeed, double rightSpeed);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_setMotorSpeeds(
    mut env: JNIEnv,
    _class: JClass,
    left_speed: jdouble,
    right_speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(left_speed as f64, right_speed as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Reads the distance from the ultrasonic sensor.
///
/// # Returns
///
/// The distance measured by the ultrasonic sensor in centimeters.
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error reading the sensor.
///
/// # JNI Signature
///
/// ```java
/// public static native double readDistance();
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_readDistance(
    mut env: JNIEnv,
    _class: JClass,
) -> jdouble {
    let mut sensors = SENSORS.lock().unwrap();
    match sensors.read_distance(50, 3, 190_000) {
        Ok(distance) => distance as jdouble,
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("{}", e));
            -1.0
        }
    }
}

/// Reads the state of a button pin.
///
/// # Arguments
///
/// * `pin` - The GPIO pin.
///
/// # Returns
///
/// `true` if the button is pressed, `false` otherwise.
fn read_button_state(pin: &InputPin) -> bool {
    pin.is_high()
}

/// Reads the state of the specified button.
///
/// # Arguments
///
/// * `button_id` - The ID of the button to read (0 for A, 1 for B, 2 for X, 3 for Y).
///
/// # Returns
///
/// `true` if the button is pressed, `false` otherwise.
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `IllegalArgumentException` if the button ID is invalid.
///
/// # JNI Signature
///
/// ```java
/// public static native boolean readButton(int buttonId);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_readButton(
    mut env: JNIEnv,
    _class: JClass,
    button_id: jint,
) -> jboolean {
    let state = match button_id as u8 {
        BUTTON_A => read_button_state(&BUTTON_A_INPUT),
        BUTTON_B => read_button_state(&BUTTON_B_INPUT),
        BUTTON_X => read_button_state(&BUTTON_X_INPUT),
        BUTTON_Y => read_button_state(&BUTTON_Y_INPUT),
        _ => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", "Invalid button ID");
            return 0;
        }
    };
    state as jboolean
}

/// Sets the brightness of the specified button LED.
///
/// # Arguments
///
/// * `button_led_id` - The ID of the button LED to control (0 for A, 1 for B, 2 for X, 3 for Y).
/// * `value` - Brightness value (0.0 for off, 1.0 for full brightness).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error controlling the LED.
///
/// # JNI Signature
///
/// ```java
/// public static native void setButtonLed(int buttonLedId, double value);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_setButtonLed(
    mut env: JNIEnv,
    _class: JClass,
    button_led_id: jint,
    value: jdouble,
) {
    let mut leds = BUTTONLEDS.lock().unwrap();
    if let Err(e) = leds.set_button_led(button_led_id as u8, value as f64) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Sets the color of a specific underlight.
///
/// # Arguments
///
/// * `light_id` - The ID of the underlight to control (0 to 5).
/// * `red` - Red color component (0 to 255).
/// * `green` - Green color component (0 to 255).
/// * `blue` - Blue color component (0 to 255).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the underlight.
///
/// # JNI Signature
///
/// ```java
/// public static native void setUnderlight(int lightId, int red, int green, int blue);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_setUnderlight(
    mut env: JNIEnv,
    _class: JClass,
    light_id: jint,
    red: jint,
    green: jint,
    blue: jint,
) {
    let mut leds = SN3218LEDS.lock().unwrap();
    if let Err(e) = leds.set_rgb(
        light_id as u8,
        red as u8,
        green as u8,
        blue as u8,
    ) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    } else if let Err(e) = leds.update_underlighting() {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Fills all underlights with the specified color.
///
/// # Arguments
///
/// * `red` - Red color component (0 to 255).
/// * `green` - Green color component (0 to 255).
/// * `blue` - Blue color component (0 to 255).
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error setting the underlights.
///
/// # JNI Signature
///
/// ```java
/// public static native void fillUnderlighting(int red, int green, int blue);
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_fillUnderlighting(
    mut env: JNIEnv,
    _class: JClass,
    red: jint,
    green: jint,
    blue: jint,
) {
    let mut leds = SN3218LEDS.lock().unwrap();
    if let Err(e) = leds.fill_underlight((red as u8, green as u8, blue as u8)) {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Clears all underlighting by turning off all underlight LEDs.
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error clearing the underlights.
///
/// # JNI Signature
///
/// ```java
/// public static native void clearUnderlighting();
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_clearUnderlighting(
    mut env: JNIEnv,
    _class: JClass,
) {
    let mut leds = SN3218LEDS.lock().unwrap();
    if let Err(e) = leds.clear_underlighting() {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

/// Captures an image from the camera and returns it as a byte array.
///
/// # Returns
///
/// A Java byte array containing the JPEG image data.
///
/// # Safety
///
/// This function interacts with hardware through JNI calls and must be used carefully.
///
/// # Errors
///
/// Throws a Java `Exception` if there is an error capturing the image.
///
/// # JNI Signature
///
/// ```java
/// public static native byte[] captureImage();
/// ```
#[no_mangle]
pub extern "system" fn Java_com_swiftbot_NativeBindings_captureImage(
    mut env: JNIEnv,
    _class: JClass,
) -> jbyteArray {
    let mut camera = CAMERA.lock().unwrap();
    match camera.capture_image() {
        Ok(image_data) => {
            //Convert Rust Vec<u8> to Java byte array
            let buf = env.byte_array_from_slice(&image_data).unwrap();
            **buf
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("{}", e));
            //Return null in case of error
            std::ptr::null_mut()
        }
    }
}