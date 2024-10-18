# SwiftBot-Control

This repository contains a Rust library (`swiftbot_rs_lib`) that provides hardware control for the SwiftBot robot platform running on a Raspberry Pi, as well as the Java wrapper for it, to provide an additional abstraction layer. The library interfaces with the robot's hardware components such as motors, LEDs, buttons, sensors, and camera, and exposes functionality through the Java Native Interface (JNI) for use in Java applications.

This library assumes all the pins are *pre-mapped*, as would be expected. However, if you would like a more customized configuration for your pins, you may want to modify the `config.rs` file.

## Table of Contents
- [Requirements](#requirements)
- [Camera Setup](#camera-setup)
- [Examples](#examples)
- [Notes](#notes)
- [License](#license)

## Requirements

The project is built on Rust and Java. It uses OpenCV for camera control.

```bash
# Update package lists
sudo apt update

# Install GStreamer development libraries (for video streaming)
sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
sudo apt install libgstrtspserver-1.0-dev  # For RTSP streaming with GStreamer

# Install OpenCV development libraries (4.x or latest from apt)
sudo apt install libopencv-dev python3-opencv

# Check the installed OpenCV version
pkg-config --modversion opencv4

# Install Rustup (Rust package manager)
# Don't use sudo here, as Rust installs to the user environment. Sudo'ing might cause issues with RCE from JetBrains or similar IDEs.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install OpenJDK (Java Development Kit for OpenCV Java bindings)
sudo apt install openjdk-17-jdk

# Install LLVM, Clang, and setup environment variables for OpenCV compilation with Rust bindings
sudo apt install llvm clang libclang-dev
export LLVM_CONFIG_PATH=/usr/bin/llvm-config
export LIBCLANG_PATH=/usr/lib/llvm-14/lib/libclang.so

# Install OpenCV dependencies required to build from source
sudo apt install build-essential cmake git pkg-config libgtk-3-dev \
                 libavcodec-dev libavformat-dev libswscale-dev \
                 libv4l-dev libxvidcore-dev libx264-dev \
                 libjpeg-dev libpng-dev libtiff-dev \
                 gfortran openexr libatlas-base-dev \
                 python3-dev python3-numpy libtbbmalloc2 \
                 libdc1394-dev

# Clone the OpenCV and OpenCV contrib repositories
cd ~
git clone https://github.com/opencv/opencv.git
git clone https://github.com/opencv/opencv_contrib.git

# Checkout the latest stable branch for both repositories (4.x)
cd opencv && git checkout 4.x
cd ../opencv_contrib && git checkout 4.x

# Create build directory and configure OpenCV with CMake
cd ~/opencv
mkdir build && cd build
cmake -D CMAKE_BUILD_TYPE=RELEASE \
      -D CMAKE_INSTALL_PREFIX=/usr/local \
      -D OPENCV_EXTRA_MODULES_PATH=~/opencv_contrib/modules \
      -D WITH_JAVA=ON \
      -D BUILD_SHARED_LIBS=OFF \
      -D OPENCV_ENABLE_NONFREE=ON \
      -D INSTALL_C_EXAMPLES=ON \
      -D INSTALL_PYTHON_EXAMPLES=ON \
      -D BUILD_EXAMPLES=ON ..

# Build OpenCV using all available CPU cores
make -j$(nproc)

# Install OpenCV
sudo make install

# Verify installation by checking for the Java bindings
sudo find /usr/local/lib -name 'libopencv_java490.so'
```

## Camera Setup

### Raspbian

On Raspbian distributions, this is fairly simple:

Install the required libs:

```bash
sudo apt update
sudo apt install libcamera-apps v4l-utils
```

After this, you can immediately access the camera and test it:

```bash
libcamera-still -o test_image.jpg
```

Configure the camera:

```bash
sudo raspi-config
```

Go to `Interfacing Options` -> `Camera` and make sure it is enabled. After that, perform `sudo reboot`, if it was not.

### Other Distributions

On other distributions, the `raspi-config` is not available, and we need to do things manually.

```bash
sudo apt update
sudo apt install libraspberrypi-bin libraspberrypi-dev # Install the RPi base libs
```

APT does not have the `libcamera-apps` in its registry, you will need to manually build and install that, too:

```bash
sudo apt install -y meson ninja-build libboost-program-options-dev libgnutls28-dev openssl python3-pip pkg-config libevent-dev
sudo apt install python3-ply
# ----------- 

git clone https://git.libcamera.org/libcamera/libcamera.git
cd libcamera

# ----------

meson build
ninja -C build
sudo ninja -C build install

# Verify

libcamera-still --version
```

Verify that all is running smooth by checking if the kernel driver is up:
```bash
sudo modprobe bcm2835-v4l2
echo "bcm2835-v4l2" | sudo tee -a /etc/modules
```

To make sure that the module loads at boot, add it to `/etc/modules`. Following that, you can use the camera as usual.

### Video Recordings & Memory Streaming

For video processing, in an attempt to accelerate it as much as possible, we will be utilizing hardware acceleration & `mmap`s for efficient memory access.

Firstly, let's address the hardware acceleration. In this library, it is done via `gstreamer`:

```bash
sudo apt-get update
sudo apt-get install -y \
    libgstreamer1.0-dev \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-omx \
    gstreamer1.0-alsa
```

`gstreamer` proves to be plentiful for proper, high-framerate recording on native Rust code. The primary issue arises when attempting to link this to JNI...
- JNI is not optimized for large data transfers between native code and itself.
- If we do go ahead with frequently copying the data directly from native code and JNI, we run into massive overhead, preventing good framerates.

Currently, there are a couple of major pathways for solving this issue, that need performance testing:
- Using **memory-mapped files** - `mmap`s to create spaces in memory that both Java and Rust can access. This is overly complex, however.
- Using high-performance IPCs like `ZeroMQ`
- RTSP streaming with GStreamer over Web Sockets - this has shown very promising results.
- Direct shared memory with JNI.

**NOTE:** You might want to see the video capture in real time. In this case, do the following:

```bash
sudo apt install --no-install-recommends raspberrypi-ui-mods lightdm
sudo apt install lxterminal #Or xterm
sudo reboot
startx
```

This will give you a minimal GUI for seeing video capture.

## Features

- **Motor Control**: Move the robot forward, backward, turn left, and turn right with adjustable speeds.
- **Sensor Reading**: Read distance measurements from the ultrasonic sensor.
- **Button Interaction**: Read the state of the buttons on the robot.
- **LED Control**: Control the brightness of button LEDs and set colors for underlighting LEDs.
- **Camera Capture**: Capture images using the Raspberry Pi Camera V2 with the `rscam` crate.

## Hardware Requirements

- **Raspberry Pi 4**
- **Raspberry Pi Camera V2**
- **HC-SR04 Ultrasonic Sensor**
- **SN3218 LED Driver** for underlighting

## Prerequisites

- **Operating System**: Any Linux Distro (SwiftBot runs on a custom barebones Yocto distribution)
- **Rust**:

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Java Development Kit (JDK)**: Version 8 or higher installed on the Raspberry Pi
- **JNI Development Headers**: Included with the JDK
- **System Libraries**:

  ```sh
  sudo apt-get update
  sudo apt-get install libclang-dev libudev-dev
  ```

- **Camera Dependencies**:

  - Ensure that the Raspberry Pi Camera is enabled in `raspi-config`.
  - Verify that `/dev/video0` exists.

## Installation

### Building the Rust Library

Build the library as a C-compatible dynamic library for JNI support:

```sh
cargo build --release
```

The compiled library will be located at `target/release/libswiftbot_rs_lib.so`.

## Usage

### Building the Rust Library

Ensure that the `Cargo.toml` file is configured correctly:

```toml
[package]
name = "swiftbot_rs_lib"
version = "1.0.0"
edition = "2021"

[lib]
name = "swiftbot_rs_lib"
crate-type = ["cdylib"]  # Compile as a C-compatible dynamic library for JNI support

[dependencies]
rppal = "0.19.0"
jni = { version = "0.21.1", features = ["invocation"] }
lazy_static = "1.4.0"
rscam = "0.5.5"
```

## Examples

### Moving the Robot Forward

```java
SwiftBot robot = new SwiftBot();
robot.forward(0.5); //Move forward at half speed
Thread.sleep(2000); //Move for 2 seconds
robot.stop();
```

### Reading Distance from Ultrasonic Sensor

```java
double distance = robot.readDistance();
System.out.println("Distance: " + distance + " cm");
```

### Checking if a Button is Pressed

```java
if (robot.isButtonPressed(SwiftBot.BUTTON_A)) {
    System.out.println("Button A is pressed!");
}
```

### Controlling Button LEDs

```java
robot.setButtonLed(SwiftBot.BUTTON_A, 1.0); //Set Button A LED to full brightness
robot.setButtonLed(SwiftBot.BUTTON_B, 0.5); //Set Button B LED to half brightness
```

### Setting Underlighting Colors

```java
//Set underlight 0 to red
robot.setUnderlight(0, 255, 0, 0);

//Fill all underlights with blue
robot.fillUnderlighting(0, 0, 255);
```

### Capturing an Image

```java
byte[] imageData = robot.captureImage();
if (imageData != null) {
    try (FileOutputStream fos = new FileOutputStream("captured_image.jpg")) {
        fos.write(imageData);
        System.out.println("Image saved as captured_image.jpg");
    } catch (IOException e) {
        e.printStackTrace();
    }
} else {
    System.out.println("Failed to capture image.");
}
```

## Notes

- **Hardware Interaction**: This library interacts directly with hardware components. Ensure that you have proper permissions and that your user is part of the `gpio` and `i2c` groups.

  ```sh
  sudo usermod -aG gpio,i2c $USER
  ```

- **Thread Safety**: The Rust library uses `Mutex` locks to ensure thread safety. Avoid long-running operations while holding a lock to prevent blocking other threads.

- **Error Handling**: JNI functions throw Java exceptions in case of errors. Make sure to handle these exceptions in your Java code.

- **Camera Usage**: The `rscam` crate is not actively maintained. Compatibility may vary depending on your Raspberry Pi OS version and kernel. Ensure that the camera is properly connected and enabled.

- **Performance Considerations**: Minimize the frequency of JNI calls in performance-critical sections to reduce overhead. The code has yet to be thoroughly tested & benchmarked with the camera module. OpenCV may be required for future use as an alternative.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
