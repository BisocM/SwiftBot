package bisocm.swiftbot.app;

import bisocm.swiftbot.lib.SwiftBot;
import org.opencv.core.Mat;
import org.opencv.imgcodecs.Imgcodecs;
import org.opencv.videoio.VideoCapture;

public class Main {
    public static void main(String[] args) {
        ButtonEventHandler buttonEventHandler = new ButtonEventHandler();
        SwiftBot bot = new SwiftBot(buttonEventHandler);

        try {
            System.out.println("Testing motor control...");

            // Move forward
            bot.forward(0.5);
            System.out.println("Moving forward...");
            Thread.sleep(1000);

            // Move backward
            bot.backward(0.5);
            System.out.println("Moving backward...");
            Thread.sleep(1000);

            // Turn left
            bot.turnLeft(0.5);
            System.out.println("Turning left...");
            Thread.sleep(1000);

            // Turn right
            bot.turnRight(0.5);
            System.out.println("Turning right...");
            Thread.sleep(1000);

            // Stop
            bot.stop();
            System.out.println("Stopping...");
            Thread.sleep(1000);

            // Set individual motor speeds
            bot.setMotorSpeeds(0.7, 0.3);
            System.out.println("Setting individual motor speeds...");
            Thread.sleep(1000);
            bot.stop();

            System.out.println("\nTesting sensor readings...");

            // Read distance
            double distance = bot.readDistance();
            System.out.println("Distance sensor reading: " + distance + " cm");

            System.out.println("\nTesting LED control...");

            // Set LED brightness
            bot.setButtonLed(SwiftBot.BUTTON_A, 0.5);
            System.out.println("Set Button A LED to half brightness.");
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_B, 0.5);
            System.out.println("Set Button B LED to half brightness.");
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_Y, 0.5);
            System.out.println("Set Button Y LED to half brightness.");
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_X, 0.5);
            System.out.println("Set Button X LED to half brightness.");
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_Y, 0.0);
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_X, 0.0);
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_A, 0.0);
            Thread.sleep(200);

            bot.setButtonLed(SwiftBot.BUTTON_B, 0.0);
            Thread.sleep(200);

            System.out.println("Testing RTSP server functionality...");
            System.out.println("Starting the RTSP server...");

            try {
                // Start the RTSP server
                bot.startRtsp();
                System.out.println("RTSP server started.");

                System.out.println("Listening on localhost:8554/stream");
                String rtspUrl = "rtsp://localhost:8554/stream"; // Replace with your RTSP URL
                VideoCapture capture = new VideoCapture(rtspUrl);

                if (!capture.isOpened()) {
                    System.err.println("Failed to open RTSP stream.");
                    return;
                }

                Mat frame = new Mat();
                int frameCount = 0;

                System.out.println("Processing frames...");
                while (true) {
                    if (capture.read(frame)) {
                        // Process the frame (e.g., convert or analyze)
                        // For example, save every 30th frame as an image
                        if (frameCount % 30 == 0) {
                            String filename = "frame_" + frameCount + ".jpg";
                            Imgcodecs.imwrite(filename, frame);
                            System.out.println("Saved " + filename);
                        }

                        frameCount++;
                    } else {
                        System.err.println("Failed to read frame.");
                        break;
                    }

                    // Optional: Control the frame rate
                    try {
                        Thread.sleep(33); // Approximately 30 FPS
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }

                capture.release();
            } finally {
                // Stop the RTSP server even if an exception occurs
                bot.stopRtsp();
                System.out.println("RTSP server stopped.");
            }

        } catch (Exception e) {
            System.err.println("An error occurred during testing: " + e.getMessage());
        }
    }
}