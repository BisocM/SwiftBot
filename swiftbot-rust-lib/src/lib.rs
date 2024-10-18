#[macro_use]
extern crate lazy_static;
mod buttons;
mod camera;
mod config;
mod motors;
mod sensors;
mod sn3218;
mod utils;
mod rtsp_streamer;

use crate::buttons::{notify_button_pressed, notify_button_released, Buttons};
use crate::camera::{CameraController};
use crate::config::{BUTTON_A_PIN, BUTTON_B_PIN, BUTTON_X_PIN, BUTTON_Y_PIN};
use crate::motors::Motors;
use crate::sensors::Sensors;
use crate::sn3218::UnderlightLeds;

use jni::objects::{JByteBuffer, JClass, JString};
use jni::sys::{jboolean, jbyteArray, jdouble, jint};
use jni::JNIEnv;
use rppal::gpio::{Gpio, InputPin};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::rtsp_streamer::{start_rtsp_streaming, stop_rtsp_streaming};

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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_stop(
    mut env: JNIEnv,
    _class: JClass,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(0.0, 0.0) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_forward(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(speed as f64, speed as f64) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_backward(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(-speed as f64, -speed as f64) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_turnLeft(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(-speed as f64, speed as f64) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_turnRight(
    mut env: JNIEnv,
    _class: JClass,
    speed: jdouble,
) {
    let mut motors = MOTORS.lock().unwrap();
    if let Err(e) = motors.set_motor_speeds(speed as f64, -speed as f64) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_setMotorSpeeds(
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_readDistance(
    mut env: JNIEnv,
    _class: JClass,
) -> jdouble {
    let mut sensors = SENSORS.lock().unwrap();
    match sensors.read_distance(50) {
        Ok(distance) => distance as jdouble,
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("{}", e));
            -1.0
        }
    }
}

//
//
//                        BUTTONS
//
//

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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_readButton(
    mut env: JNIEnv,
    _class: JClass,
    button_id: jint,
) -> jboolean {
    let state = match button_id {
        0 => read_button_state(&BUTTON_A_INPUT),
        1 => read_button_state(&BUTTON_B_INPUT),
        2 => read_button_state(&BUTTON_X_INPUT),
        3 => read_button_state(&BUTTON_Y_INPUT),
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_setButtonLed(
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_setUnderlight(
    mut env: JNIEnv,
    _class: JClass,
    light_id: jint,
    red: jint,
    green: jint,
    blue: jint,
) {
    let mut leds = SN3218LEDS.lock().unwrap();
    if let Err(e) = leds.set_rgb(light_id as u8, red as u8, green as u8, blue as u8) {
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_fillUnderlighting(
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_clearUnderlighting(
    mut env: JNIEnv,
    _class: JClass,
) {
    let mut leds = SN3218LEDS.lock().unwrap();
    if let Err(e) = leds.clear_underlighting() {
        let _ = env.throw_new("java/lang/Exception", format!("{}", e));
    }
}

#[no_mangle]
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_startButtonMonitoring(
    mut env: JNIEnv,
    _class: JClass,
) {
    let jvm = env.get_java_vm().unwrap(); // Get Java VM instance for attaching threads later
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let button_pins: [InputPin; 4] = [
        gpio.get(BUTTON_A_PIN).unwrap().into_input_pulldown(),
        gpio.get(BUTTON_B_PIN).unwrap().into_input_pulldown(),
        gpio.get(BUTTON_X_PIN).unwrap().into_input_pulldown(),
        gpio.get(BUTTON_Y_PIN).unwrap().into_input_pulldown(),
    ];

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    // Monitoring thread
    let tx_clone = Arc::clone(&tx);
    thread::spawn(move || {
        let mut button_states = [false; 4];
        loop {
            for (i, pin) in button_pins.iter().enumerate() {
                let is_pressed = pin.is_high();
                let tx = tx_clone.lock().unwrap();

                if is_pressed && !button_states[i] {
                    tx.send((i as u8, true)).unwrap();
                    button_states[i] = true;
                } else if !is_pressed && button_states[i] {
                    tx.send((i as u8, false)).unwrap();
                    button_states[i] = false;
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Notification thread
    thread::spawn(move || {
        while let Ok((button_id, pressed)) = rx.recv() {
            if pressed {
                notify_button_pressed(&jvm, button_id);
            } else {
                notify_button_released(&jvm, button_id);
            }
        }
    });
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
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_captureImageToBuffer(
    mut env: JNIEnv,
    _class: JClass,
    buffer: JByteBuffer,
) -> jint {
    let mut camera = CAMERA.lock().unwrap();

    // Get the buffer address
    let buf = match unsafe { env.get_direct_buffer_address(&buffer) } {
        Ok(buf) => buf,
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("Failed to get buffer address: {}", e));
            return -1;
        }
    };

    // Get the buffer capacity
    let buffer_len = match env.get_direct_buffer_capacity(&buffer) {
        Ok(capacity) => capacity as usize,
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("Failed to get buffer capacity: {}", e));
            return -1;
        }
    };

    // Call capture_image with both buffer and buffer_len
    match camera.capture_image(buf, buffer_len) {
        Ok(data_len) => data_len as jint,
        Err(e) => {
            let _ = env.throw_new("java/lang/Exception", format!("{}", e));
            -1
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_startRtspStreaming(
    mut env: JNIEnv,
    _class: JClass,
) {
    if let Err(e) = start_rtsp_streaming() {
        let _ = env.throw_new("java/lang/Exception", format!("Failed to start RTSP streaming: {}", e));
    }
}

#[no_mangle]
pub extern "system" fn Java_bisocm_swiftbot_lib_NativeBindings_stopRtspStreaming(
    _env: JNIEnv,
    _class: JClass,
) {
    stop_rtsp_streaming();
}