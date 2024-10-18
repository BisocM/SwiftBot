use std::sync::{Arc, Mutex};
use gstreamer as gst;
use gstreamer_rtsp_server as gst_rtsp_server;
use gstreamer_rtsp_server::prelude::*;
use std::thread;
use glib;
use lazy_static::lazy_static;

lazy_static! {
    static ref RTSP_MAIN_LOOP: Arc<Mutex<Option<glib::MainLoop>>> = Arc::new(Mutex::new(None));
}

pub fn start_rtsp_streaming() -> Result<(), glib::BoolError> {
    gst::init().unwrap();

    // Create an RTSP server
    let server = gst_rtsp_server::RTSPServer::new();
    let mount_points = server.mount_points().unwrap();

    // Define the GStreamer pipeline string for the stream
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    factory.set_launch(
        "rpicamsrc bitrate=1000000 ! video/x-h264, width=640, height=480, framerate=30/1 ! \
        h264parse ! rtph264pay name=pay0 pt=96 config-interval=1",
    );

    // Mount the pipeline on the RTSP server
    mount_points.add_factory("/stream", factory);

    // Attach the server to the default main context
    let _id = server.attach(None)?; // No need to map the error

    println!("RTSP streaming started at rtsp://localhost:8554/stream");

    // Run the GLib main loop to handle the streaming
    let main_loop = glib::MainLoop::new(None, false);
    {
        let mut main_loop_guard = RTSP_MAIN_LOOP.lock().unwrap();
        *main_loop_guard = Some(main_loop.clone());
    }

    thread::spawn(move || {
        main_loop.run();
    });

    Ok(())
}

pub fn stop_rtsp_streaming() {
    let main_loop_guard = RTSP_MAIN_LOOP.lock().unwrap();
    if let Some(ref main_loop) = *main_loop_guard {
        main_loop.quit();
        println!("RTSP streaming stopped.");
    } else {
        println!("RTSP streaming is not running.");
    }
}
