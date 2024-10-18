use gstreamer as gst;
use gstreamer::parse;
use gstreamer::prelude::*;
use lazy_static::lazy_static;
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
    videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY},
    Result,
};
use std::error::Error;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CAMERA: Mutex<CameraController> = Mutex::new(
        CameraController::new().expect("Failed to initialize CameraController")
    );
}

pub struct CameraController {
    capture: VideoCapture,
}

impl CameraController {
    /// Initializes the camera with the specified resolution and format.
    pub fn new() -> Result<Self> {
        // Use CAP_ANY to let OpenCV select the backend
        let mut capture = VideoCapture::new(0, CAP_ANY)?; // Device index 0

        // Set resolution
        capture.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
        capture.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;

        // Set framerate
        capture.set(opencv::videoio::CAP_PROP_FPS, 30.0)?;

        // Check if the capture is opened
        let is_opened = capture.is_opened()?;
        if !is_opened {
            return Err(opencv::Error::new(
                opencv::core::StsError,
                "Failed to open camera".to_string(),
            ));
        }

        Ok(CameraController { capture })
    }

    /// Captures a single image from the camera into the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable byte slice where the image data will be stored.
    ///
    /// # Returns
    ///
    /// The size of the image data written to the buffer.
    pub fn capture_image(&mut self, buffer: *mut u8, buffer_len: usize) -> Result<usize> {
        let mut frame = Mat::default();

        // Capture a frame
        self.capture.read(&mut frame)?;

        // Check if the frame is empty
        if frame.empty() {
            return Err(opencv::Error::new(
                opencv::core::StsError,
                "Failed to capture frame".to_string(),
            ));
        }

        // Encode the image to JPEG format
        let mut buf = Vector::<u8>::new();
        let params = Vector::<i32>::new(); // No additional params

        imgcodecs::imencode(".jpg", &frame, &mut buf, &params)?;

        let data_len = buf.len();

        unsafe {
            // Convert the raw pointer to a mutable slice
            let buffer_slice = std::slice::from_raw_parts_mut(buffer, buffer_len);

            if buffer_slice.len() < data_len {
                return Err(opencv::Error::new(
                    opencv::core::StsOutOfRange,
                    format!(
                        "Buffer too small: required {}, but got {}",
                        data_len,
                        buffer_slice.len()
                    ),
                ));
            }

            // Copy data to the provided buffer
            buffer_slice[..data_len].copy_from_slice((&buf).as_ref());
        }

        Ok(data_len)
    }
}