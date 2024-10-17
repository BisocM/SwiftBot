use rscam::{Camera, Config};
use std::error::Error;

pub struct CameraController {
    camera: Camera,
}

impl CameraController {
    /// Initializes the camera with the specified resolution and format.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut camera = Camera::new("/dev/video0")?;

        //Configure the camera (adjust resolution and format as needed)
        camera.start(&Config {
            interval: (1, 30), // 30 fps
            resolution: (640, 480),
            format: b"MJPG",
            ..Default::default()
        })?;

        Ok(CameraController { camera })
    }

    /// Captures a single image from the camera.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the JPEG image data.
    pub fn capture_image(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        //Capture a frame
        let frame = self.camera.capture()?;

        //The frame data is in MJPEG format (JPEG)
        let image_data = frame.to_vec();

        Ok(image_data)
    }

    /// Stops the camera and releases resources.
    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.camera.stop()?;
        Ok(())
    }
}