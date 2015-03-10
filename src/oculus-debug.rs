#![crate_name = "oculus-debug"]

extern crate ovr;

use ovr::{RenderGLConfig, DistortionCapabilities, TrackingCapabilities, Ovr, HmdType};

fn main() {
    let ovr = match Ovr::init() {
        Some(ovr) => ovr,
        None => {
            println!("Could not initialize Oculus SDK");
            return;
        }
    };

    let hmd = match ovr.create_hmd_debug(HmdType::DK2) {
        Some(hmd) => hmd,
        None => {
            println!("Could not create debug hmd");
            return;
        }
    };
    println!("Hmd Type: {:?}", hmd.hmd_type);
    println!("Product Name: {:?}", hmd.product_name);
    println!("Manufacturer: {:?}", hmd.manufacturer);
    println!("Hmd Capabilities: {:?}", hmd.hmd_capabilities);
    println!("Tracking Capabilities: {:?}", hmd.tracking_capabilities);
    println!("Distortion Capabilities: {:?}", hmd.distortion_capabilities);
    println!("Resolution: {:?}", hmd.resolution);
    println!("Window Position: {:?}", hmd.window_position);
    println!("right: {:?}", hmd.default_eye_fov.right);
    println!("left {:?}", hmd.default_eye_fov.left);
    println!("Eyes render order: [{:?}, {:?}]", hmd.eye_render_order[0], hmd.eye_render_order[1]);
    println!("Display device name: {:?}", hmd.display_device_name);
    println!("Display id: {:?}", hmd.display_id);

    // ovrHmd_GetFovTextureSize()
    let started = hmd.configure_tracking(hmd.tracking_capabilities, hmd.tracking_capabilities);

    if !started {
        println!("Could not start sensor");
        return;
    }

    // let render_conf = RenderGLConfig {
    //     size: hmd.resolution,
    //     multisample: 0,
    //     display: None,
    //     window: None,
    // };
    //
    // let eye_renderers = hmd.configure_rendering(
    //     &render_conf,
    //     hmd.distortion_capabilities,
    //     hmd.default_eye_fov
    //     );
}
