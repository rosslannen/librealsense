const EXAMPLES: [&str; 25] = [
    "../../../examples/ar-basic/rs-ar-basic.cpp",
    "../../../examples/ar-advanced/rs-ar-advanced.cpp",
    "../../../examples/tracking-and-depth/rs-tracking-and-depth.cpp",
    "../../../examples/gl/rs-gl.cpp",
    "../../../examples/pose/rs-pose.cpp",
    "../../../examples/pose-predict/rs-pose-predict.cpp",
    "../../../examples/pose-and-image/rs-pose-and-image.cpp",
    "../../../examples/trajectory/rs-trajectory.cpp",
    "../../../examples/motion/rs-motion.cpp",
    "../../../examples/sensor-control/rs-sensor-control.cpp",
    "../../../examples/measure/rs-measure.cpp",
    "../../../examples/C/depth/rs-depth.c",
    "../../../examples/C/color/rs-color.c",
    "../../../examples/C/distance/rs-distance.c",
    "../../../examples/post-processing/rs-post-processing.cpp",
    "../../../examples/align-advanced/rs-align-advanced.cpp",
    "../../../examples/hello-realsense/rs-hello-realsense.cpp",
    "../../../examples/software-device/rs-software-device.cpp",
    "../../../examples/capture/rs-capture.cpp",
    "../../../examples/callback/rs-callback.cpp",
    "../../../examples/save-to-disk/rs-save-to-disk.cpp",
    "../../../examples/multicam/rs-multicam.cpp",
    "../../../examples/pointcloud/rs-pointcloud.cpp",
    "../../../examples/align/rs-align.cpp",
    "../../../examples/record-playback/rs-record-playback.cpp",
];

const TOOLS: [&str; 18] = [
    "../../../tools/benchmark/rs-benchmark.cpp",
    "../../../tools/rosbag-inspector/rs-rosbag-inspector.cpp",
    "../../../tools/depth-quality/rs-depth-quality.cpp",
    "../../../tools/depth-quality/depth-quality-model.cpp",
    "../../../tools/realsense-viewer/realsense-viewer.cpp",
    "../../../tools/convert/rs-convert.cpp",
    "../../../tools/enumerate-devices/rs-enumerate-devices.cpp",
    "../../../tools/fw-logger/rs-fw-logger.cpp",
    "../../../tools/fw-logger/fw-log-data.cpp",
    "../../../tools/fw-logger/fw-logs-formating-options.cpp",
    "../../../tools/fw-logger/fw-logs-parser.cpp",
    "../../../tools/fw-logger/fw-logs-xml-helper.cpp",
    "../../../tools/fw-logger/string-formatter.cpp",
    "../../../tools/terminal/rs-terminal.cpp",
    "../../../tools/terminal/auto-complete.cpp",
    "../../../tools/recorder/rs-record.cpp",
    "../../../tools/fw-update/rs-fw-update.cpp",
    "../../../tools/data-collect/rs-data-collect.cpp",
];

const SQLITE3: [&str; 1] = ["../../../third-party/sqlite/sqlite3.c"];

const ROSBAG: [&str; 19] = [
    "../../../third-party/realsense-file/rosbag/console_bridge/src/console.cpp",
    "../../../third-party/realsense-file/rosbag/cpp_common/src/debug.cpp",
    "../../../third-party/realsense-file/rosbag/cpp_common/src/header.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/bag.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/bag_player.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/buffer.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/chunked_file.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/lz4_stream.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/message_instance.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/query.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/stream.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/uncompressed_stream.cpp",
    "../../../third-party/realsense-file/rosbag/rosbag_storage/src/view.cpp",
    "../../../third-party/realsense-file/rosbag/roscpp_serialization/src/serialization.cpp",
    "../../../third-party/realsense-file/rosbag/roslz4/src/lz4s.c",
    "../../../third-party/realsense-file/rosbag/roslz4/src/xxhash.c",
    "../../../third-party/realsense-file/rosbag/rostime/src/duration.cpp",
    "../../../third-party/realsense-file/rosbag/rostime/src/rate.cpp",
    "../../../third-party/realsense-file/rosbag/rostime/src/time.cpp",
];

const LZ4: [&str; 1] = ["../../../third-party/realsense-file/lz4/lz4.c"];

const GLFW: [&str; 16] = [
    "../../../third-party/glfw/src/context.c",
    "../../../third-party/glfw/src/init.c",
    "../../../third-party/glfw/src/input.c",
    "../../../third-party/glfw/src/monitor.c",
    "../../../third-party/glfw/src/vulkan.c",
    "../../../third-party/glfw/src/window.c",
    "../../../third-party/glfw/src/x11_init.c",
    "../../../third-party/glfw/src/x11_monitor.c",
    "../../../third-party/glfw/src/x11_window.c",
    "../../../third-party/glfw/src/xkb_unicode.c",
    "../../../third-party/glfw/src/posix_time.c",
    "../../../third-party/glfw/src/posix_thread.c",
    "../../../third-party/glfw/src/glx_context.c",
    "../../../third-party/glfw/src/egl_context.c",
    "../../../third-party/glfw/src/osmesa_context.c",
    "../../../third-party/glfw/src/linux_joystick.c",
];

const EASYLOGGINGPP: [&str; 1] = ["../../../third-party/easyloggingpp/src/easylogging++.cc"];

const GLAD: [&str; 1] = ["../../../third-party/glad/glad.c"];

const IMGUI: [&str; 12] = [
    "../../../third-party/imgui/imgui.cpp",
    "../../../third-party/imgui/imgui_draw.cpp",
    "../../../third-party/imgui/imgui_impl_glfw.cpp",
    "../../../third-party/imgui/imgui.cpp",
    "../../../third-party/imgui/imgui_draw.cpp",
    "../../../third-party/imgui/imgui_impl_glfw.cpp",
    "../../../third-party/imgui/imgui.cpp",
    "../../../third-party/imgui/imgui_draw.cpp",
    "../../../third-party/imgui/imgui_impl_glfw.cpp",
    "../../../third-party/imgui/imgui.cpp",
    "../../../third-party/imgui/imgui_draw.cpp",
    "../../../third-party/imgui/imgui_impl_glfw.cpp",
];

const TINYFILEDIALOGS: [&str; 1] = ["../../../third-party/tinyfiledialogs/tinyfiledialogs.c"];

// No idea what "common" means...
const COMMON: [&str; 12] = [
    "../../../common/opengl3.cpp",
    "../../../common/model-views.cpp",
    "../../../common/notifications.cpp",
    "../../../common/viewer.cpp",
    "../../../common/ux-window.cpp",
    "../../../common/ux-alignment.cpp",
    "../../../common/rs-config.cpp",
    "../../../common/os.cpp",
    "../../../common/fw-update-helper.cpp",
    "../../../common/metadata-helper.cpp",
    "../../../common/on-chip-calib.cpp",
    "../../../common/fw/empty.c",
];

const BUILD: [&str; 4] = [
    "../../../build/common/fw/D4XX_FW_Image.c",
    "../../../build/common/fw/D4XX_RC_Image.c",
    "../../../build/common/fw/SR3XX_FW_Image.c",
    "../../../build/common/fw/target.c",
];

const SOURCES: [&str; 109] = [
    // "../../../src/verify.c",
    "../../../src/ds5/ds5-options.cpp",
    "../../../src/ds5/ds5-timestamp.cpp",
    "../../../src/ds5/ds5-private.cpp",
    "../../../src/ds5/ds5-motion.cpp",
    "../../../src/ds5/ds5-nonmonochrome.cpp",
    "../../../src/ds5/ds5-device.cpp",
    "../../../src/ds5/ds5-color.cpp",
    "../../../src/ds5/ds5-active.cpp",
    "../../../src/ds5/ds5-factory.cpp",
    "../../../src/ds5/ds5-fw-update-device.cpp",
    "../../../src/ds5/advanced_mode/rs_advanced_mode.cpp",
    "../../../src/ds5/advanced_mode/presets.cpp",
    "../../../src/ds5/advanced_mode/advanced_mode.cpp",
    "../../../src/ds5/ds5-auto-calibration.cpp",
    "../../../src/ivcam/ivcam-private.cpp",
    "../../../src/ivcam/sr300.cpp",
    "../../../src/ivcam/sr300-fw-update-device.cpp",
    "../../../src/l500/l500-depth.cpp",
    "../../../src/l500/l500-private.cpp",
    "../../../src/l500/l500-color.cpp",
    "../../../src/l500/l500-device.cpp",
    "../../../src/l500/l500-motion.cpp",
    "../../../src/l500/l500-factory.cpp",
    "../../../src/l500/l500-fw-update-device.cpp",
    "../../../src/l500/l500-serializable.cpp",
    "../../../src/l500/l500-options.cpp",
    "../../../src/media/record/record_device.cpp",
    "../../../src/media/record/record_sensor.cpp",
    "../../../src/media/playback/playback_device.cpp",
    "../../../src/media/playback/playback_sensor.cpp",
    "../../../src/media/ros/ros_reader.cpp",
    "../../../src/media/ros/ros_writer.cpp",
    "../../../src/mock/sql.cpp",
    "../../../src/mock/recorder.cpp",
    "../../../src/proc/sse/sse-align.cpp",
    "../../../src/proc/sse/sse-pointcloud.cpp",
    "../../../src/proc/processing-blocks-factory.cpp",
    "../../../src/proc/align.cpp",
    "../../../src/proc/colorizer.cpp",
    "../../../src/proc/pointcloud.cpp",
    "../../../src/proc/occlusion-filter.cpp",
    "../../../src/proc/synthetic-stream.cpp",
    "../../../src/proc/syncer-processing-block.cpp",
    "../../../src/proc/decimation-filter.cpp",
    "../../../src/proc/spatial-filter.cpp",
    "../../../src/proc/temporal-filter.cpp",
    "../../../src/proc/hole-filling-filter.cpp",
    "../../../src/proc/disparity-transform.cpp",
    "../../../src/proc/y8i-to-y8y8.cpp",
    "../../../src/proc/y12i-to-y16y16.cpp",
    "../../../src/proc/identity-processing-block.cpp",
    "../../../src/proc/threshold.cpp",
    "../../../src/proc/rates-printer.cpp",
    "../../../src/proc/zero-order.cpp",
    "../../../src/proc/units-transform.cpp",
    "../../../src/proc/rotation-transform.cpp",
    "../../../src/proc/color-formats-converter.cpp",
    "../../../src/proc/depth-formats-converter.cpp",
    "../../../src/proc/motion-transform.cpp",
    "../../../src/proc/auto-exposure-processor.cpp",
    "../../../src/proc/depth-decompress.cpp",
    "../../../src/pipeline/pipeline.cpp",
    "../../../src/pipeline/config.cpp",
    "../../../src/pipeline/profile.cpp",
    "../../../src/pipeline/aggregator.cpp",
    "../../../src/fw-update/fw-update-device.cpp",
    "../../../src/fw-update/fw-update-factory.cpp",
    "../../../src/fw-update/fw-update-unsigned.cpp",
    "../../../src/libusb/context-libusb.cpp",
    "../../../src/libusb/interface-libusb.cpp",
    "../../../src/libusb/device-libusb.cpp",
    "../../../src/libusb/messenger-libusb.cpp",
    "../../../src/libusb/request-libusb.cpp",
    "../../../src/libusb/enumerator-libusb.cpp",
    "../../../src/linux/backend-v4l2.cpp",
    "../../../src/linux/backend-hid.cpp",
    "../../../src/tm2/tm-device.cpp",
    "../../../src/tm2/tm-info.cpp",
    "../../../src/algo.cpp",
    "../../../src/archive.cpp",
    "../../../src/backend.cpp",
    "../../../src/context.cpp",
    "../../../src/device.cpp",
    "../../../src/device_hub.cpp",
    "../../../src/environment.cpp",
    "../../../src/error-handling.cpp",
    "../../../src/global_timestamp_reader.cpp",
    "../../../src/hw-monitor.cpp",
    "../../../src/image.cpp",
    "../../../src/image-avx.cpp",
    "../../../src/log.cpp",
    "../../../src/option.cpp",
    "../../../src/rs.cpp",
    "../../../src/sensor.cpp",
    "../../../src/software-device.cpp",
    "../../../src/source.cpp",
    "../../../src/stream.cpp",
    "../../../src/sync.cpp",
    "../../../src/types.cpp",
    "../../../src/frame-validator.cpp",
    "../../../src/gl/synthetic-stream-gl.cpp",
    "../../../src/gl/yuy2rgb-gl.cpp",
    "../../../src/gl/pointcloud-gl.cpp",
    "../../../src/gl/rs-gl.cpp",
    "../../../src/gl/pc-shader.cpp",
    "../../../src/gl/camera-shader.cpp",
    "../../../src/gl/upload.cpp",
    "../../../src/gl/colorizer-gl.cpp",
    "../../../src/gl/align-gl.cpp",
];

fn main() {
    // This needs to go away at some point, but for now...
    println!("cargo:rerun-if-changed=build.rs");

    let library = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("libusb-1.0")
        .unwrap();

    // Build sqlite3: Confirmed works!
    cc::Build::new()
        .files(SQLITE3.iter())
        .include("../../../third-party/sqlite/")
        .flag("-Wno-cast-function-type")
        .flag("-Wno-implicit-fallthrough")
        .compile("sqlite3");

    // Build rosbag: Confirmed works!
    cc::Build::new()
        .cpp(true)
        .files(ROSBAG.iter())
        .include("../../../third-party/realsense-file/rosbag/rosbag_storage/include/")
        .include("../../../third-party/realsense-file/rosbag/roscpp_traits/include/")
        .include("../../../third-party/realsense-file/rosbag/rostime/include/")
        .include("../../../third-party/realsense-file/rosbag/rostime/include/")
        .include("../../../third-party/realsense-file/rosbag/roscpp_serialization/include/")
        .include("../../../third-party/realsense-file/rosbag/cpp_common/include/")
        .include("../../../third-party/realsense-file/rosbag/msgs/")
        .include("../../../third-party/realsense-file/rosbag/console_bridge/include/")
        .include("../../../third-party/realsense-file/rosbag/roslz4/include/")
        .include("../../../third-party/realsense-file/boost/")
        .flag("-Wno-comment")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-parentheses")
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-deprecated")
        .flag("-Wno-unused-variable")
        .flag("-Wno-catch-value")
        .flag("-Wno-placement-new")
        .flag("-Wno-misleading-indentation")
        .flag("-fpermissive")
        .compile("rosbag");

    // Build lz4: Confirmed works
    cc::Build::new()
        .files(LZ4.iter())
        .include("../../../third-party/realsense-file/lz4/")
        .compile("lz4");

    // Build easyloggingpp
    cc::Build::new()
        .files(EASYLOGGINGPP.iter())
        .include("../../../third-party/easyloggingpp/src/")
        .flag("-Wno-unused-parameter")
        .compile("easyloggingpp");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    cc::Build::new()
        .cpp(true)
        .define("RS2_USE_V4L2_BACKEND", None)
        .define("BUILD_EASYLOGGINGPP", None)
        .include("../../../include/")
        .include("../../../src/")
        .include("../../../third-party/realsense-file/rosbag/rosbag_storage/include/")
        .include("../../../third-party/realsense-file/rosbag/roscpp_traits/include/")
        .include("../../../third-party/realsense-file/rosbag/rostime/include/")
        .include("../../../third-party/realsense-file/rosbag/rostime/include/")
        .include("../../../third-party/realsense-file/rosbag/roscpp_serialization/include/")
        .include("../../../third-party/realsense-file/rosbag/cpp_common/include/")
        .include("../../../third-party/realsense-file/rosbag/msgs/")
        .include("../../../third-party/realsense-file/boost/")
        .include("../../../third-party/glad/")
        .include("../../../third-party/glfw/include/")
        .include("../../../third-party/realsense-file/lz4/")
        .include("../../../third-party/sqlite/")
        .include("../../../common/")
        .include(&library.include_paths[0])
        .files(SOURCES.iter())
        .flag("-Wno-unknown-pragmas")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-reorder")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-variable")
        .flag("-Wno-ignored-qualifiers")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-catch-value")
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-comment")
        .flag("-Wno-parentheses")
        .flag("-Wno-switch")
        .flag("-Wno-pessimizing-move")
        .flag("-Wno-redundant-move")
        .flag("-Wno-unused-function")
        .flag("-Wno-deprecated-copy")
        .flag("-Wno-placement-new")
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-misleading-indentation")
        .flag("-Wno-shift-negative-value")
        .flag("-Wno-empty-body")
        .flag("-Wno-address")
        .flag("-Wno-int-in-bool-context")
        .flag("-Wno-nonnull-compare")
        .flag("-Wno-unused-value")
        .flag("-Wno-deprecated")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-type-limits")
        .static_flag(true)
        .shared_flag(false)
        .flag(&format!("-L{}", out_dir))
        .flag("-l:libsqlite3.a")
        .flag("-l:librosbag.a")
        .flag("-l:liblz4.a")
        .flag("-l:libeasyloggingpp.a")
        .compile("realsense");
}
