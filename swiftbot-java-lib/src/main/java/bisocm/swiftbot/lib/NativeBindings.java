package bisocm.swiftbot.lib;

class NativeBindings {

    /***********************************************************************
     *                         STATIC INITIALIZER                          *
     ***********************************************************************
     * This static block loads the native Rust library when the class is
     * first loaded. It allows Java to call Rust functions defined in the
     * native library `swiftbot_rs_lib`.
     ***********************************************************************/
    static {
        System.loadLibrary("swiftbot_rs_lib"); //Load the Rust native library
        //It is recommended to set an environment variable that points to the compiled .so lib for flexibility.
        //Make a quick python script that runs on init and links all that is required.
    }

    /***********************************************************************
     *                     BUTTON LISTENER CALLBACK                        *
     ***********************************************************************
     * The `ButtonListener` object handles button press and release events.
     * Java registers a listener using `registerButtonListener` and Rust
     * triggers `onButtonPressed` and `onButtonReleased` when a button state
     * changes.
     ***********************************************************************/
    private static ButtonListener buttonListener;

    /***********************************************************************
     *                REGISTER BUTTON LISTENER CALLBACK                    *
     ***********************************************************************
     * `registerButtonListener` registers the listener instance in Java,
     * which will handle button events from Rust. It also calls the Rust
     * function `startButtonMonitoring` to start monitoring the buttons.
     ***********************************************************************/
    public static void registerButtonListener(ButtonListener listener) {
        buttonListener = listener;
        startButtonMonitoring(); //Start the monitoring thread in Rust
    }

    /***********************************************************************
     *                 CALLBACK METHOD: BUTTON PRESSED                    *
     ***********************************************************************
     * This method is called from Rust to notify Java when a button is
     * pressed. It passes the `buttonId` to the registered listener.
     ***********************************************************************/
    public static void onButtonPressed(int buttonId) {
        if (buttonListener != null) {
            buttonListener.onButtonPressed(buttonId);
        }
    }

    /***********************************************************************
     *                CALLBACK METHOD: BUTTON RELEASED                     *
     ***********************************************************************
     * This method is called from Rust to notify Java when a button is
     * released. It also passes the `buttonId` to the registered listener.
     ***********************************************************************/
    public static void onButtonReleased(int buttonId) {
        if (buttonListener != null) {
            buttonListener.onButtonReleased(buttonId);
        }
    }

    /***********************************************************************
     *                        NATIVE METHODS SECTION                       *
     ***********************************************************************
     * Native methods define Java interfaces for functions implemented
     * in Rust. This includes `startButtonMonitoring`, which initiates
     * monitoring for button events in Rust, as well as methods for motor,
     * sensor, LED, and camera control.
     ***********************************************************************/
    private static native void startButtonMonitoring(); //Starts the Rust thread for button monitoring

    //Motor control
    public static native void stop();
    public static native void forward(double speed);
    public static native void backward(double speed);
    public static native void turnLeft(double speed);
    public static native void turnRight(double speed);
    public static native void setMotorSpeeds(double leftSpeed, double rightSpeed);

    //Sensor readings
    public static native double readDistance();
    public static native boolean readButton(int buttonId);

    //LED control
    public static native void setButtonLed(int buttonLedId, double value);
    public static native void setUnderlight(int lightId, int red, int green, int blue);
    public static native void fillUnderlighting(int red, int green, int blue);
    public static native void clearUnderlighting();

    //Camera control
    public static native byte[] captureImage();
}