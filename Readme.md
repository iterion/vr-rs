VR-RS
=====

[![Build Status](https://travis-ci.org/csherratt/vr-rs.svg?branch=master)](https://travis-ci.org/csherratt/vr-rs)

VR-RS providers bindings for the Oculus's libovr. Currently it only provides bindings for 0.3.2c.

Building
--------

vr-rs is fully cargoized. to Compile

    cargo build

To add as a dependency using cargo Cargo add the following to your `Cargo.toml`

    [dependencies.ovr]
    git = "https://github.com/csherratt/vr-rs.git"


Using VR-RS
-----------

# Initializing

```rust

extern crate ovr = "oculus-vr";

use ovr::{SensorCapabilities, Ovr};

fn main() {
    // Initalize the Oculus VR library
    let ovr = match Ovr::init() {
        Some(ovr) => ovr,
        None => {
             println!("Could not initialize Oculus SDK");
            return;           
        }
    };

    // get the first available HMD device, returns None
    // if no HMD device is currently plugged in
    let hmd = match ovr.first_hmd() {
        Some(hmd) => hmd,
        None => {
            println!("Could not get hmd");
            return;
        }
    };

    // start the sensor recording, Require orientation tracking
    let started = hmd.start_sensor(SensorCapabilities::new().set_orientation(true),
                                   SensorCapabilities::new().set_orientation(true));
    if !started {
        println!("Could not start sensor");
        return;
    }
}
```

# Render loop

The Oculus SDK will handle most of the heavy lifting of the barrel distortion.

```rust
fn render(frame_index: uint, hmd: &ovr::Hmd, base_view: &Matrix4<f32>) {
    // start a new frame, the frame_index should increment each frame
    let frame_timing = hmd.begin_frame(frame_index);
    let desc = hmd.get_description();

    for &eye in [ovr::EyeLeft, ovr::EyeRight].iter() {
        // start rendering a new eye, this will give the most current
        // copy of the pose from the HMD tracking sensor
        let pose = self.window.get_hmd().begin_eye_render(eye);

        // base_view * pose * eye_view_adjustment
        let view = base_view.mul_m(&pose.orientation.to_matrix4())
                            .mul_m(&Matrix4::translate(&eye.view_adjust));
        let projection = desc.eye_fovs.eye(eye).default_eye_fov;

        // render to texture
        render();

        let texture = ovr::Texture(width, height,
                                   viewport_offset_x, viewport_offset_y,
                                   viewport_width, viewport_height,
                                   opengl_texture_id);
        hmd.end_eye_render(eye, pose, &texture);
    }

    // this will swap the buffers and frame sync
    hmd.end_frame();
}
```
