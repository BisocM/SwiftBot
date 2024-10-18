package bisocm.swiftbot.lib;

import java.lang.annotation.Native;

public class SwiftBot {
    //TODO: Fix the button listener. It is completely borked at the moment.
    private ButtonListener buttonListener;

    public SwiftBot(ButtonListener buttonListener)
    {
        //Register the button listener with Rust.
        registerButtonListener(buttonListener);
    }

    /***********************************************************************
     *                          BUTTON CONSTANTS                           *
     ***********************************************************************
     * Constants for button identifiers used in this class for readability
     * and easy referencing in method calls.
     ***********************************************************************/
    public static final int BUTTON_A = 0;
    public static final int BUTTON_B = 1;
    public static final int BUTTON_X = 2;
    public static final int BUTTON_Y = 3;

    /***********************************************************************
     *                     MOTOR CONTROL METHODS                           *
     ***********************************************************************
     * Methods to control the movement of the SwiftBot, including forward,
     * backward, turning, and stopping. Each method interacts with the
     * motors via native calls to the Rust backend.
     ***********************************************************************/

    /**
     * Moves the SwiftBot forward at the specified speed.
     *
     * @param speed The speed to move forward, ranging from 0.0 (stationary)
     *              to 1.0 (maximum speed).
     */
    public void forward(double speed) {
        NativeBindings.forward(speed);
    }

    /**
     * Moves the SwiftBot backward at the specified speed.
     *
     * @param speed The speed to move backward, ranging from 0.0 (stationary)
     *              to 1.0 (maximum speed).
     */
    public void backward(double speed) {
        NativeBindings.backward(speed);
    }

    /**
     * Turns the SwiftBot to the left at the specified speed.
     *
     * @param speed The speed to turn left, ranging from 0.0 (no turn) to 1.0
     *              (maximum turn speed).
     */
    public void turnLeft(double speed) {
        NativeBindings.turnLeft(speed);
    }

    /**
     * Turns the SwiftBot to the right at the specified speed.
     *
     * @param speed The speed to turn right, ranging from 0.0 (no turn) to 1.0
     *              (maximum turn speed).
     */
    public void turnRight(double speed) {
        NativeBindings.turnRight(speed);
    }

    /**
     * Stops the SwiftBot's movement immediately by setting motor speeds to 0.
     */
    public void stop() {
        NativeBindings.stop();
    }

    /**
     * Sets the speeds of the left and right motors independently for more
     * precise control.
     *
     * @param leftSpeed  Speed for the left motor (-1.0 for full reverse,
     *                   1.0 for full forward).
     * @param rightSpeed Speed for the right motor (-1.0 for full reverse,
     *                   1.0 for full forward).
     */
    public void setMotorSpeeds(double leftSpeed, double rightSpeed) {
        NativeBindings.setMotorSpeeds(leftSpeed, rightSpeed);
    }

    /***********************************************************************
     *                       SENSOR READING METHOD                         *
     ***********************************************************************
     * Method to read the current distance to an object using the
     * ultrasonic sensor on the SwiftBot.
     ***********************************************************************/

    /**
     * Reads and returns the distance from the SwiftBot to the nearest object.
     *
     * @return The distance in centimeters. Returns -1.0 if an error occurs
     *         (such as when the sensor is unavailable).
     */
    public double readDistance() {
        return NativeBindings.readDistance();
    }

    /***********************************************************************
     *                       BUTTON INTERACTION METHODS                    *
     ***********************************************************************
     * Method to check the current state (pressed or not pressed) of a
     * specific button on the SwiftBot.
     ***********************************************************************/

    /**
     * Checks if a specified button is currently pressed.
     *
     * @param buttonId The ID of the button to check (e.g., BUTTON_A, BUTTON_B).
     * @return `true` if the button is pressed, `false` otherwise.
     */
    public boolean isButtonPressed(int buttonId) {
        return NativeBindings.readButton(buttonId);
    }

    /**
     * Registers the current SwiftBot class instance to listen for button presses.
     *
     */
    private void registerButtonListener(ButtonListener buttonListener){
        this.buttonListener = buttonListener;
        NativeBindings.registerButtonListener(buttonListener);
    }

    public void onButtonReleased(int buttonId) {
        NativeBindings.onButtonReleased(buttonId);
    }

    public static void onButtonPressed(int buttonId) {
        NativeBindings.onButtonPressed(buttonId);
    }

    /***********************************************************************
     *                         LED CONTROL METHOD                          *
     ***********************************************************************
     * Method to control the LED brightness for a specified button on the
     * SwiftBot.
     ***********************************************************************/

    /**
     * Sets the brightness level for a button LED.
     *
     * @param buttonLedId The ID of the button LED (e.g., BUTTON_A, BUTTON_B).
     * @param value       Brightness level, where 0.0 is off and 1.0 is full
     *                    brightness.
     */
    public void setButtonLed(int buttonLedId, double value) {
        NativeBindings.setButtonLed(buttonLedId, value);
    }

    /***********************************************************************
     *                    UNDERLIGHT (SN3218) CONTROL METHODS              *
     ***********************************************************************
     * Methods to control the SN3218 underlighting on the SwiftBot, allowing
     * customization of color and brightness.
     ***********************************************************************/

    /**
     * Sets the color of an individual underlight LED.
     *
     * @param lightId The ID of the underlight LED (range may vary by model).
     * @param red     Red component (0 to 255).
     * @param green   Green component (0 to 255).
     * @param blue    Blue component (0 to 255).
     */
    public void setUnderlight(int lightId, int red, int green, int blue) {
        NativeBindings.setUnderlight(lightId, red, green, blue);
    }

    /**
     * Sets all underlights to the specified color.
     *
     * @param red   Red component (0 to 255).
     * @param green Green component (0 to 255).
     * @param blue  Blue component (0 to 255).
     */
    public void fillUnderlighting(int red, int green, int blue) {
        NativeBindings.fillUnderlighting(red, green, blue);
    }

    /**
     * Clears all underlights, turning them off.
     */
    public void clearUnderlighting() {
        NativeBindings.clearUnderlighting();
    }

    /***********************************************************************
     *                         CAMERA CAPTURE METHOD                       *
     ***********************************************************************
     * Method to capture an image from the SwiftBot's camera and return it
     * as a byte array.
     ***********************************************************************/

    /**
     * Captures an image from the SwiftBot's camera.
     *
     * @return A byte array containing the captured image data in JPEG format.
     *         Returns `null` if an error occurs.
     */
    public byte[] captureImage() {
        return NativeBindings.captureImageToBuffer();
    }

    public void startRtsp() {
        NativeBindings.startRtspStreaming();
    }

    public void stopRtsp() {
        NativeBindings.stopRtspStreaming();
    }
}