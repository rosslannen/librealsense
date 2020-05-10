use librealsense_sys as ffi;

use std::ffi::CStr;

fn check_error(e: *mut ffi::rs2_error) {
    unsafe {
        if !e.is_null() {
            let raw_failed_function = ffi::rs2_get_failed_function(e);
            let failed_function = CStr::from_ptr(raw_failed_function);

            let raw_failed_args = ffi::rs2_get_failed_args(e);
            let failed_args = CStr::from_ptr(raw_failed_args);

            let raw_message = ffi::rs2_get_error_message(e);
            let message = CStr::from_ptr(raw_message);

            println!(
                "rs_error was raised when calling {:?}({:?}):",
                failed_function, failed_args
            );
            println!("    {:?}", message);
            panic!();
        }
    }
}

// These parameters are reconfigurable
const STREAM: u32 = ffi::rs2_stream_RS2_STREAM_DEPTH;
const FORMAT: u32 = ffi::rs2_format_RS2_FORMAT_Z16;
const WIDTH: i32 = 640;
const HEIGHT: i32 = 0;
const FPS: i32 = 30;
const STREAM_INDEX: i32 = 0;

unsafe fn print_device_info(device: *mut ffi::rs2_device) {
    let mut e: *mut ffi::rs2_error = std::ptr::null_mut();

    let raw_name =
        ffi::rs2_get_device_info(device, ffi::rs2_camera_info_RS2_CAMERA_INFO_NAME, &mut e);
    check_error(e);
    let name = CStr::from_ptr(raw_name);
    println!("Using device: {:?}", name);

    let raw_serial_number = ffi::rs2_get_device_info(
        device,
        ffi::rs2_camera_info_RS2_CAMERA_INFO_SERIAL_NUMBER,
        &mut e,
    );
    check_error(e);
    let serial_number = CStr::from_ptr(raw_serial_number);
    println!("    Serial number: {:?}", serial_number);

    let raw_firmware_version = ffi::rs2_get_device_info(
        device,
        ffi::rs2_camera_info_RS2_CAMERA_INFO_FIRMWARE_VERSION,
        &mut e,
    );
    check_error(e);
    let firmware_version = CStr::from_ptr(raw_firmware_version);
    println!("    Firmware version: {:?}", firmware_version);
}

fn main() {
    unsafe {
        let mut e: *mut ffi::rs2_error = std::ptr::null_mut();

        // Create a context object.
        let ctx = ffi::rs2_create_context(ffi::RS2_API_VERSION, &mut e);
        check_error(e);

        // Get a list of all connected devices.
        let device_list = ffi::rs2_query_devices(ctx, &mut e);
        check_error(e);

        let device_count = ffi::rs2_get_device_count(device_list, &mut e);
        check_error(e);
        println!("There are {} connected RealSense devices.", device_count);
        if device_count == 0 {
            return;
        }

        // Get the first connected device.
        let device = ffi::rs2_create_device(device_list, 0, &mut e);
        print_device_info(device);

        // Create a pipeline to configure, start, and stop camera streaming.
        let pipeline = ffi::rs2_create_pipeline(ctx, &mut e);
        check_error(e);

        // Create a config instance to specify hardware configuration.
        let config = ffi::rs2_create_config(&mut e);
        check_error(e);

        // Request a certain configuration
        ffi::rs2_config_enable_stream(
            config,
            STREAM,
            STREAM_INDEX,
            WIDTH,
            HEIGHT,
            FORMAT,
            FPS,
            &mut e,
        );
        check_error(e);

        // Start the pipeline streaming
        let pipeline_profile = ffi::rs2_pipeline_start_with_config(pipeline, config, &mut e);
        if !e.is_null() {
            println!("The connected device doesn't support depth streaming!");
            panic!()
        }

        loop {
            // Wait until a new composite frame is available
            let frames =
                ffi::rs2_pipeline_wait_for_frames(pipeline, ffi::RS2_DEFAULT_TIMEOUT, &mut e);
            check_error(e);

            // Get the number of frames embedded within the composite frame
            let num_of_frames = ffi::rs2_embedded_frames_count(frames, &mut e);
            check_error(e);

            for i in 0..num_of_frames {
                let frame = ffi::rs2_extract_frame(frames, i, &mut e);
                check_error(e);

                // Check if the given frame can be extended to the depth interface.
                let is_extendable_to_depth = ffi::rs2_is_frame_extendable_to(
                    frame,
                    ffi::rs2_extension_RS2_EXTENSION_DEPTH_FRAME,
                    &mut e,
                );
                if 0 == is_extendable_to_depth {
                    continue;
                }

                // Get the depth frame's dimensions
                let width = ffi::rs2_get_frame_width(frame, &mut e);
                check_error(e);
                let height = ffi::rs2_get_frame_height(frame, &mut e);
                check_error(e);

                // Query the distance from the camera to the object in the center of the image.
                let dist_to_center =
                    ffi::rs2_depth_frame_get_distance(frame, width / 2, height / 2, &mut e);
                check_error(e);

                println!(
                    "The camera is facing an object {} meters away.",
                    dist_to_center
                );

                ffi::rs2_release_frame(frame);
            }

            ffi::rs2_release_frame(frames);
        }

        // Stop the pipeline streaming.
        ffi::rs2_pipeline_stop(pipeline, &mut e);
        check_error(e);

        // Release resources
        ffi::rs2_delete_pipeline_profile(pipeline_profile);
        ffi::rs2_delete_config(config);
        ffi::rs2_delete_pipeline(pipeline);
        ffi::rs2_delete_device(device);
        ffi::rs2_delete_device_list(device_list);
        ffi::rs2_delete_context(ctx);
    }
}
