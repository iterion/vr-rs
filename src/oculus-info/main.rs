#![crate_name = "oculus-info"]

extern crate ovr;

use ovr::{TrackingCapabilities, Ovr};

fn main() {
    let ovr = match Ovr::init() {
        Some(ovr) => ovr,
        None => {
             println!("Could not initialize Oculus SDK");
            return;           
        }
    };

    let hmd = match ovr.first_hmd() {
        Some(hmd) => hmd,
        None => {
            println!("Could not get hmd");
            return;
        }
    };
    // ovrHmd_GetFovTextureSize()
    let started = hmd.configure_tracking(TrackingCapabilities::new().set_orientation(true),
                                         TrackingCapabilities::new().set_orientation(true));

    if !started {
        println!("Could not start sensor");
        return;
    }


    // match hmd.get_tracking_state() {
    //     Some(sd) => {
    //         println!("Vendor id: {:x}", sd.vendor_id);
    //         println!("Product id: {:x}", sd.product_id);
    //         println!("Serial number: {:?}", sd.serial_number);
    //     }
    //     None => println!("Failed to get sensor description"),
    // }

    // let hmd_desc = hmd.get_description();
    //
    // println!("Hmd Type: {:?}", hmd_desc.hmd_type);
    // println!("Product Name: {:?}", hmd_desc.product_name);
    // println!("Manufacture: {:?}", hmd_desc.manufacture);
    // println!("Hmd Capabilities: {:?}", hmd_desc.hmd_capabilities);
    // println!("Tracking Capabilities: {:?}", hmd_desc.sensor_capabilities);
    // println!("Distortion Capabilities: {:?}", hmd_desc.distortion_capabilities);
    // println!("Resolution: {:?}", hmd_desc.resolution);
    // println!("Window Position: {:?}", hmd_desc.window_position);
    // println!("right: {:?}", hmd_desc.eye_fovs.right);
    // println!("left {:?}", hmd_desc.eye_fovs.left);
    // println!("Eyes render order: [{:?}, {:?}]", hmd_desc.eye_render_order[0], hmd_desc.eye_render_order[1]);
    // println!("Display device name: {:?}", hmd_desc.display_device_name);
    // println!("Display id: {:?}", hmd_desc.display_id);
}
