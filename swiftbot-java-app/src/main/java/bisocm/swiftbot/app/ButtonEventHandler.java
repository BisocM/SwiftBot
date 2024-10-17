package bisocm.swiftbot.app;

import bisocm.swiftbot.lib.ButtonListener;

public class ButtonEventHandler implements ButtonListener {
    @Override
    public void onButtonPressed(int buttonId) {
        System.out.println("Button " + buttonId + " pressed.");
    }

    @Override
    public void onButtonReleased(int buttonId) {
        System.out.println("Button " + buttonId + " released.");
    }
}