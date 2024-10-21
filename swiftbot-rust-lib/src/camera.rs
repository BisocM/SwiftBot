use lazy_static::lazy_static;
use std::ffi::c_void;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use jni::objects::JByteBuffer;
use jni::JNIEnv;

use v4l::buffer::Type;
use v4l::format::FourCC;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const CHANNELS: usize = 3; //For RGB
const BUFFER_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize) * CHANNELS;
const FRAME_RATE: u32 = 30; //Desired frame rate

lazy_static! {
    static ref CAMERA_MUTEX: Mutex<()> = Mutex::new(());
    static ref CAMERA_RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref NATIVE_BUFFER: Arc<Mutex<*mut u8>> = Arc::new(Mutex::new(ptr::null_mut::<u8>()));
}

pub fn get_direct_buffer<'a>(env: &mut JNIEnv<'a>) -> Result<JByteBuffer<'a>, String> {
    let _camera_lock = CAMERA_MUTEX.lock().map_err(|e| format!("Failed to lock camera: {}", e))?;

    let mut buffer_guard = NATIVE_BUFFER.lock().unwrap();
    if (*buffer_guard).is_null() {
        //Allocate the native buffer
        unsafe {
            *buffer_guard = libc::malloc(BUFFER_SIZE) as *mut u8;
            if (*buffer_guard).is_null() {
                return Err("Failed to allocate native buffer.".to_string());
            }
        }

        //Start the camera
        start_camera()?;
    }

    //Return the DirectByteBuffer wrapping the native buffer
    let buffer = unsafe {
        env.new_direct_byte_buffer(*buffer_guard, BUFFER_SIZE)
    };
    match buffer {
        Ok(buf) => Ok(buf),
        Err(e) => Err(format!("Failed to create DirectByteBuffer: {}", e)),
    }
}

pub fn release_camera() {
    stop_camera();
    //Clean up the native buffer
    let mut buffer_guard = NATIVE_BUFFER.lock().unwrap();
    unsafe {
        if !(*buffer_guard).is_null() {
            libc::free(*buffer_guard as *mut c_void);
            *buffer_guard = ptr::null_mut();
        }
    }
}

pub fn capture_video(file_path: &str, duration_seconds: u32) -> Result<(), String> {
    let _camera_lock = CAMERA_MUTEX.lock().map_err(|e| format!("Failed to lock camera: {}", e))?;

    //Ensure the camera is not already running
    if *CAMERA_RUNNING.lock().unwrap() {
        return Err("Camera is currently in use.".to_string());
    }

    //Open the camera device
    let device = Device::new(0).map_err(|e| format!("Failed to open device: {}", e))?;

    //Set camera parameters
    let mut format = device.format().map_err(|e| format!("Failed to get format: {}", e))?;
    format.width = WIDTH;
    format.height = HEIGHT;
    format.fourcc = FourCC::new(b"YUYV"); //Use YUYV format

    device.set_format(&format).map_err(|e| format!("Failed to set format: {}", e))?;

    let mut stream = MmapStream::with_buffers(&device, Type::VideoCapture, 4)
        .map_err(|e| format!("Failed to create stream: {}", e))?;

    //Open the output file
    let output_file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut writer = BufWriter::new(output_file);

    //Use the jpeg-encoder crate
    use jpeg_encoder::{ColorType, Encoder};

    let start_time = Instant::now();
    let frame_duration = Duration::from_secs_f64(1.0 / FRAME_RATE as f64);

    while start_time.elapsed().as_secs() < duration_seconds as u64 {
        let frame_start = Instant::now();

        let (data, _) = stream.next().map_err(|e| format!("Failed to capture frame: {}", e))?;

        let mut rgb_buffer = vec![0u8; BUFFER_SIZE];
        unsafe {
            yuyv422_to_rgb24(&data, rgb_buffer.as_mut_ptr());
        }

        //Encode the RGB buffer into a JPEG image
        let mut jpeg_data = Vec::new();
        let mut encoder = Encoder::new(&mut jpeg_data, 90);
        encoder.encode(&rgb_buffer, WIDTH as u16, HEIGHT as u16, ColorType::Rgb)
            .map_err(|e| format!("Failed to encode JPEG: {}", e))?;

        //Write the JPEG image to the file
        writer
            .write_all(&jpeg_data)
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        //Sleep for the remainder of the frame duration if necessary
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    writer.flush().map_err(|e| format!("Failed to flush writer: {}", e))?;

    Ok(())
}

fn start_camera() -> Result<(), String> {
    let mut running_guard = CAMERA_RUNNING.lock().unwrap();
    if *running_guard {
        return Ok(());
    }

    //Open the camera device
    let device = Device::new(0).map_err(|e| format!("Failed to open device: {}", e))?;

    //Set camera parameters
    let mut format = device.format().map_err(|e| format!("Failed to get format: {}", e))?;
    format.width = WIDTH;
    format.height = HEIGHT;
    format.fourcc = FourCC::new(b"YUYV"); //Use YUYV format

    device.set_format(&format).map_err(|e| format!("Failed to set format: {}", e))?;

    //Create a stream for capturing frames
    let stream = MmapStream::with_buffers(&device, Type::VideoCapture, 4)
        .map_err(|e| format!("Failed to create stream: {}", e))?;

    //Clone variables to move into thread
    let buffer_clone = Arc::clone(&NATIVE_BUFFER);
    let running_clone = Arc::clone(&CAMERA_RUNNING);

    thread::spawn(move || {
        let mut stream = stream;
        {
            let mut running_guard = running_clone.lock().unwrap();
            *running_guard = true;
        }

        while *running_clone.lock().unwrap() {
            match stream.next() {
                Ok((data, _)) => {
                    let buffer_guard = buffer_clone.lock().unwrap();
                    let native_buffer: *mut u8 = *buffer_guard;
                    if !native_buffer.is_null() {
                        unsafe {
                            yuyv422_to_rgb24(&data, native_buffer);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Capture error: {}", e);
                    thread::sleep(Duration::from_millis(10));
                }
            }

            //Sleep briefly to avoid tight loop
            thread::sleep(Duration::from_millis(1));
        }

        //Clean up
        let mut running_guard = running_clone.lock().unwrap();
        *running_guard = false;
    });

    Ok(())
}

fn stop_camera() {
    let mut running_guard = CAMERA_RUNNING.lock().unwrap();
    if !*running_guard {
        return;
    }
    *running_guard = false;

    //Wait for the camera thread to finish
    drop(running_guard);
    thread::sleep(Duration::from_millis(100));
}

unsafe fn yuyv422_to_rgb24(src: &[u8], dest: *mut u8) {
    let width = WIDTH as usize;
    let height = HEIGHT as usize;

    let mut i = 0; //Index in src
    let mut j = 0; //Index in dest

    while i + 3 < src.len() && j + 5 < width * height * 3 {
        let y0 = src[i] as f32;
        let u = src[i + 1] as f32 - 128.0;
        let y1 = src[i + 2] as f32;
        let v = src[i + 3] as f32 - 128.0;

        //First pixel
        let c = y0 - 16.0;

        let r = (1.164 * c + 1.596 * v).round().clamp(0.0, 255.0);
        let g = (1.164 * c - 0.392 * u - 0.813 * v).round().clamp(0.0, 255.0);
        let b = (1.164 * c + 2.017 * u).round().clamp(0.0, 255.0);

        *dest.add(j) = r as u8;
        *dest.add(j + 1) = g as u8;
        *dest.add(j + 2) = b as u8;

        //Second pixel
        let c = y1 - 16.0;

        let r = (1.164 * c + 1.596 * v).round().clamp(0.0, 255.0);
        let g = (1.164 * c - 0.392 * u - 0.813 * v).round().clamp(0.0, 255.0);
        let b = (1.164 * c + 2.017 * u).round().clamp(0.0, 255.0);

        *dest.add(j + 3) = r as u8;
        *dest.add(j + 4) = g as u8;
        *dest.add(j + 5) = b as u8;

        i += 4;
        j += 6;
    }
}