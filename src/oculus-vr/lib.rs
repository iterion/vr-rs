#![crate_name = "ovr"]
#![crate_type = "lib"]
#![feature(link_args, std_misc)]
#![allow(non_upper_case_globals)]

extern crate cgmath;
extern crate libc;

use libc::{c_int, c_uint, c_void, c_float, c_double};
use std::default::Default;
use std::ptr;
use std::old_path::BytesContainer;

use cgmath::Quaternion;
use cgmath::{Vector2, Vector3};
use cgmath::{Matrix4};

#[cfg(target_os = "linux")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name="udev")]
#[link(name="Xinerama")]
#[link(name="Xrandr")]
#[link(name="X11")]
#[link(name="GL")]
extern {}

#[cfg(target_os = "macos")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
extern {}


pub mod ll {
    use libc::{c_uint, c_int, c_float, c_char, c_void, c_double, c_short};
    use std::ptr;
    use std::default::Default;

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Vector2i {
        pub x: c_int,
        pub y: c_int
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Sizei {
        pub x: c_int,
        pub y: c_int
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Recti {
        pub pos: Vector2i,
        pub size: Sizei
    }


    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct FovPort {
        pub up_tan: c_float,
        pub down_tan: c_float,
        pub left_tan: c_float,
        pub right_tan: c_float
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Vector2f {pub x: c_float, pub y: c_float}

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Vector3f {pub x: c_float, pub y: c_float, pub z: c_float}

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Quaternionf {pub x: c_float, pub y: c_float, pub z: c_float, pub w: c_float}

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Matrix4f {pub m11: c_float, pub m12: c_float, pub m13: c_float, pub m14: c_float,
                         pub m21: c_float, pub m22: c_float, pub m23: c_float, pub m24: c_float,
                         pub m31: c_float, pub m32: c_float, pub m33: c_float, pub m34: c_float,
                         pub m41: c_float, pub m42: c_float, pub m43: c_float, pub m44: c_float}


    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct Posef {
        pub orientation: Quaternionf,
        pub position: Vector3f
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct PoseState {
        pub pose: Posef,
        pub angular_velocity: Vector3f,
        pub linear_velocity: Vector3f,
        pub angular_acceleration: Vector3f,
        pub linear_acceleration: Vector3f,
        pub time_in_seconds: c_double
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct SensorData {
        pub accelerometer: Vector3f,
        pub gyro: Vector3f,
        pub magnetometer: Vector3f,
        pub temperature: c_float,
        pub time_in_seconds: c_float
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct TrackingState {
        pub head_pose: PoseState,
        pub camera_pose: Posef,
        pub leveled_camera_pose: Posef,
        pub raw_sensor_data: SensorData,
        pub status_flags: c_uint,
        pub last_vision_processing_time: c_double,
        pub last_vision_frame_latency: c_double,
        pub last_camera_frame_counter: c_uint
    }

    #[repr(C)]
    pub struct HmdStruct;

    #[repr(C)]
    pub struct Hmd {
        pub handle: *const HmdStruct,
        pub hmd_type: c_int,
        pub product_name: *const c_char,
        pub manufacturer: *const c_char,
        pub vendor_id: c_short,
        pub product_id: c_short,
        pub serial_number: [c_char; 24],
        pub firmware_major: c_short,
        pub firmware_minor: c_short,
        pub camera_frustum_h_fov: c_float,
        pub camera_frustum_v_fov: c_float,
        pub camera_frustum_near_z: c_float,
        pub camera_frustum_far_z: c_float,
        pub hmd_capabilities: c_uint,
        pub tracking_capabilities: c_uint,
        pub distortion_capabilities: c_uint,
        pub default_eye_fov: [FovPort; 2],
        pub max_eye_fov: [FovPort; 2],
        pub eye_render_order: [c_uint; 2],
        pub resolution: Sizei,
        pub window_position: Vector2i,
        pub display_device_name: *const c_char,
        pub display_id: c_int
    }

    impl Default for Hmd {
        fn default() -> Hmd {
            Hmd {
                handle: ptr::null(),
                hmd_type: 0,
                product_name: ptr::null(),
                manufacturer: ptr::null(),
                vendor_id: 0,
                product_id: 0,
                serial_number: [0; 24],
                firmware_major: 0,
                firmware_minor: 0,
                camera_frustum_h_fov: 0f32,
                camera_frustum_v_fov: 0f32,
                camera_frustum_near_z: 0f32,
                camera_frustum_far_z: 0f32,
                hmd_capabilities: 0,
                tracking_capabilities: 0,
                distortion_capabilities: 0,
                resolution: Default::default(),
                window_position: Default::default(),
                default_eye_fov: [Default::default(); 2],
                max_eye_fov: [Default::default(); 2],
                eye_render_order: [0; 2],
                display_device_name: ptr::null(),
                display_id: 0
            }
        }
    }

    #[derive(Copy, Debug, Clone)]
    #[repr(C)]
    pub struct SensorDesc {
        pub vendor_id: c_short,
        pub product_id: c_short,
        pub serial_number: [c_char; 24]
    }

    #[derive(Clone, Default, Debug, Copy)]
    #[repr(C)]
    pub struct EyeRenderDesc {
        pub eye: c_uint,
        pub fov: FovPort,
        pub distorted_viewport: Recti,
        pub pixels_per_tan_angle_at_center: Vector2f,
        pub view_adjust: Vector3f
    }

    #[derive(Copy, Default, Debug, Clone)]
    #[repr(C)]
    pub struct RenderApiConfigHeader {
        pub render_api_type: c_uint,
        pub rt_size: Sizei,
        pub multisample: c_int,
    }

    #[repr(C)]
    pub struct RenderApiConfig {
        pub header: RenderApiConfigHeader,
        pub display: *const c_void,
        pub window: *const c_void,
        pub padd: [*const c_void; 6]
    }

    #[derive(Copy, Debug, Clone)]
    #[repr(C)]
    pub struct FrameTiming {
        pub delta_seconds: f32,
        pub this_frame_seconds: f64,
        pub timewarp_point_seconds: f64,
        pub next_frame_seconds: f64,
        pub scanout_midpoint_seconds: f64,
        pub eye_scanout_seconds: [f64; 2]        
    }

    #[derive(Copy, Default, Debug, Clone)]
    #[repr(C)]
    pub struct TextureHeader {
        pub render_api_type: c_uint,
        pub size: Sizei,
        pub viewport: Recti    
    }

    #[derive(Copy, Debug, Clone)]
    #[repr(C)]
    pub struct Texture {
        pub header: TextureHeader,
        pub texture_id: u32,
        pub padd: [*const c_void; 7]
    }

    pub const Hmd_None                      : c_int = 0;
    pub const Hmd_DK1                       : c_int = 3;
    pub const Hmd_DKHD                      : c_int = 4;
    pub const Hmd_CrystalCoveProto          : c_int = 5;
    pub const Hmd_DK2                       : c_int = 6;
    pub const Hmd_Other                     : c_int = 7;

    pub const HmdCap_Present                : c_uint = 0x0001;
    pub const HmdCap_Available              : c_uint = 0x0002;
    pub const HmdCap_LowPersistence         : c_uint = 0x0080;
    pub const HmdCap_LatencyTest            : c_uint = 0x0100;
    pub const HmdCap_DynamicPrediction      : c_uint = 0x0200;
    pub const HmdCap_NoVSync                : c_uint = 0x1000;
    pub const HmdCap_NoRestore              : c_uint = 0x4000;
    pub const HmdCap_Writable_Mask          : c_uint = 0x1380;

    pub const TrackingCap_Orientation       : c_uint = 0x0010;
    pub const TrackingCap_MagYawCorrection  : c_uint = 0x0020;
    pub const TrackingCap_Position          : c_uint = 0x0040;
    pub const TrackingCap_Idle              : c_uint = 0x0100;

    pub const Status_OrientationTracked     : c_uint = 0x0001;
    pub const Status_PositionTracked        : c_uint = 0x0002;
    pub const Status_PositionConnected      : c_uint = 0x0020;
    pub const Status_HmdConnected           : c_uint = 0x0080;

    pub const DistortionCap_Chromatic       : c_uint = 0x01;
    pub const DistortionCap_TimeWarp        : c_uint = 0x02;
    pub const DistortionCap_Vignette        : c_uint = 0x08;

    pub const Eye_Left                      : c_uint = 0;
    pub const Eye_Right                     : c_uint = 1;

    pub const RenderAPI_None                : c_uint = 0;
    pub const RenderAPI_OpenGL              : c_uint = 1;
    pub const RenderAPI_Android_GLES        : c_uint = 2;
    pub const RenderAPI_D3D9                : c_uint = 3;
    pub const RenderAPI_D3D10               : c_uint = 4;
    pub const RenderAPI_D3D11               : c_uint = 5;
    pub const RenderAPI_Count               : c_uint = 6;

    extern "C" {
        pub fn ovr_Initialize() -> bool;
        pub fn ovr_Shutdown();
        pub fn ovrHmd_Detect() -> c_int;
        pub fn ovrHmd_Create(index: c_int) -> *const Hmd;
        pub fn ovrHmd_Destroy(hmd: *const Hmd);
        pub fn ovrHmd_CreateDebug(hmd_type: c_int) -> *const Hmd;
        pub fn ovrHmd_GetLastError(hmd: *const Hmd) -> *const c_char;
        pub fn ovrHmd_GetEnabledCaps(hmd: *const Hmd) -> c_uint;
        pub fn ovrHmd_SetEnabledCaps(hmd: *const Hmd, flags: c_uint);
        pub fn ovrHmd_ConfigureTracking(hmd: *const Hmd,
                                        supported_tracking_caps: c_uint,
                                        required_tracking_caps: c_uint) -> bool;
        pub fn ovrHmd_GetTrackingState(hmd: *const Hmd, abs_time: c_double) -> TrackingState;
        pub fn ovrHmd_GetFovTextureSize(hmd: *const Hmd,
                                        eye: c_uint,
                                        fov: FovPort,
                                        pixels: c_float) -> Sizei;
        pub fn ovrHmd_ConfigureRendering(hmd: *const Hmd,
                                         apiConfig: *const RenderApiConfig,
                                         distortionCaps: c_uint,
                                         fov_in: *const FovPort,
                                         render_desc_out: *mut EyeRenderDesc) -> bool;
        pub fn ovrHmd_BeginFrame(hmd: *const Hmd,
                                 frame_index: c_uint) -> FrameTiming;
        pub fn ovrHmd_EndFrame(hmd: *const Hmd);
        pub fn ovrHmd_BeginEyeRender(hmd: *const Hmd, eye: c_uint) -> Posef;
        pub fn ovrHmd_EndEyeRender(hmd: *const Hmd, eye: c_uint, 
                                   pose: Posef, texture: *const Texture);
        pub fn ovrMatrix4f_Projection(fov: FovPort,
                                      znear: c_float,
                                      zfar: c_float,
                                      right_handed: bool) -> Matrix4f;

        pub fn ovr_WaitTillTime(abs_time: c_double) -> c_double;
        pub fn ovr_GetTimeInSeconds() -> c_double;
    }
}


pub fn get_time() -> f64 {
    unsafe{ ll::ovr_GetTimeInSeconds() as f64 }
}

pub fn wait_till_time(time: f64) -> f64 {
    unsafe{ ll::ovr_WaitTillTime(time as c_double) as f64 }
}

#[derive(Debug, Copy, Clone)]
pub enum HmdType {
    None,
    DK1,
    DKHD,
    CrystalCoveProto,
    DK2,
    Other
}

impl HmdType {
    fn from_ll(c: c_int) -> HmdType {
        match c {
            ll::Hmd_None             => HmdType::None,
            ll::Hmd_DK1              => HmdType::DK1,
            ll::Hmd_DKHD             => HmdType::DKHD,
            ll::Hmd_CrystalCoveProto => HmdType::CrystalCoveProto,
            ll::Hmd_DK2              => HmdType::DK2,
            _                        => HmdType::Other
        }
    }

    fn to_ll(&self) -> c_int {
        match *self {
            HmdType::None             => ll::Hmd_None,
            HmdType::DK1              => ll::Hmd_DK1,
            HmdType::DKHD             => ll::Hmd_DKHD,
            HmdType::CrystalCoveProto => ll::Hmd_CrystalCoveProto,
            HmdType::DK2              => ll::Hmd_DK2,
            HmdType::Other            => ll::Hmd_Other
        }
    }
}

pub struct Ovr;

impl Ovr {
    pub fn init() -> Option<Ovr> {
        unsafe {
            if ll::ovr_Initialize() {
                Some(Ovr)
            } else {
                None
            }
        }
    }

    // return a count of the number of Hmd devices
    pub fn detect(&self) -> isize {
        unsafe { ll::ovrHmd_Detect() as isize }
    }

    pub fn create_hmd(&self, index: isize) -> Option<Hmd> {
        unsafe {
            let desc = ll::ovrHmd_Create(index as i32);
            let hmd = Hmd::from_ll(desc);
            if !hmd.handle.is_null() {
                Some(hmd)
            } else {
                None
            }
        }
    }

    pub fn first_hmd(&self) -> Option<Hmd> {
        if self.detect() >= 1 {
            self.create_hmd(0)
        } else {
            None
        }
    }

    pub fn create_hmd_debug(&self, hmd_type: HmdType) -> Option<Hmd> {
        unsafe {
            let desc = ll::ovrHmd_CreateDebug(hmd_type.to_ll());
            let hmd = Hmd::from_ll(desc);
            if !hmd.handle.is_null() {
                Some(hmd)
            } else {
                None
            }
        }
    }
}

impl Drop for Ovr {
    fn drop(&mut self) {
        unsafe { ll::ovr_Shutdown(); }
    }
}

#[derive(Debug, Clone)]
pub struct Hmd {
    pub handle: *const ll::Hmd,
    pub hmd_type: HmdType,
    pub product_name: String,
    pub manufacturer: String,
    pub vendor_id: i16,
    pub product_id: i16,
    pub serial_number: String,
    pub firmware_major: i16,
    pub firmware_minor: i16,
    pub camera_frustum_h_fov: f32,
    pub camera_frustum_v_fov: f32,
    pub camera_frustum_near_z: f32,
    pub camera_frustum_far_z: f32,
    pub hmd_capabilities: HmdCapabilities,
    pub tracking_capabilities: TrackingCapabilities,
    pub distortion_capabilities: DistortionCapabilities,
    pub default_eye_fov: PerEye<FovPort>,
    pub max_eye_fov: PerEye<FovPort>,
    pub eye_render_order: [Eye; 2],
    pub resolution: ll::Sizei,
    pub window_position: ll::Vector2i,
    pub display_device_name: String,
    pub display_id: i32
}

// pub struct Hmd {
//     ptr: *mut ll::Hmd
// }
//
unsafe impl Sync for Hmd {}
unsafe impl Send for Hmd {}

impl Drop for Hmd {
    fn drop(&mut self) {
        unsafe {ll::ovrHmd_Destroy(self.handle)}
    }
}

impl Hmd {
    fn from_ll(sd: *const ll::Hmd) -> Hmd {
        unsafe {
            let hmd: &ll::Hmd = &*sd;
            Hmd {
                handle: sd,
                hmd_type: HmdType::from_ll(hmd.hmd_type),
                product_name: from_buf((hmd.product_name as *const i8) as *const u8),
                manufacturer: from_buf((hmd.manufacturer as *const i8) as *const u8),
                vendor_id: hmd.vendor_id,
                product_id: hmd.product_id,
                serial_number: "1".to_string(),
                firmware_major: hmd.firmware_major,
                firmware_minor: hmd.firmware_minor,
                camera_frustum_h_fov: hmd.camera_frustum_h_fov,
                camera_frustum_v_fov: hmd.camera_frustum_v_fov,
                camera_frustum_near_z: hmd.camera_frustum_near_z,
                camera_frustum_far_z: hmd.camera_frustum_far_z,
                hmd_capabilities: HmdCapabilities{
                    flags: hmd.hmd_capabilities
                },
                tracking_capabilities: TrackingCapabilities{
                    flags: hmd.tracking_capabilities
                },
                distortion_capabilities: DistortionCapabilities{
                    flags: hmd.distortion_capabilities
                },
                resolution: hmd.resolution,
                window_position: hmd.window_position,
                default_eye_fov: PerEye::new(
                    FovPort::from_ll(hmd.default_eye_fov[ll::Eye_Left as usize]),
                    FovPort::from_ll(hmd.default_eye_fov[ll::Eye_Right as usize])
                ),
                max_eye_fov: PerEye::new(
                    FovPort::from_ll(hmd.max_eye_fov[ll::Eye_Left as usize]),
                    FovPort::from_ll(hmd.max_eye_fov[ll::Eye_Right as usize])
                ),
                eye_render_order: [Eye::from_ll(hmd.eye_render_order[0]),
                                   Eye::from_ll(hmd.eye_render_order[1])],
                display_device_name: from_buf((hmd.display_device_name as *const i8) as *const u8),
                display_id: hmd.display_id
            }
        }
    }

    pub fn get_last_error(&self) -> Result<(), String> {
        unsafe {
            let ptr = ll::ovrHmd_GetLastError(self.handle);
            if ptr.is_null() {
                Ok(())
            } else {
                Err(from_buf(ptr as *const u8))
            }
        }
    }

    pub fn get_enabled_caps(&self) -> HmdCapabilities {
        unsafe {
            let flags = ll::ovrHmd_GetEnabledCaps(self.handle);
            HmdCapabilities{flags:flags}
        }
    }

    pub fn set_enabled_caps(&self, cap: TrackingCapabilities) {
        unsafe {
            let flags = cap.flags;
            ll::ovrHmd_SetEnabledCaps(self.handle, flags);
        }
    }

    pub fn configure_tracking(&self,
                              supported: TrackingCapabilities,
                              required: TrackingCapabilities) -> bool {
        unsafe {
            ll::ovrHmd_ConfigureTracking(self.handle, supported.flags, required.flags)
        }
    }

    pub fn get_tracking_state(&self, abs_time: f64) -> TrackingState {
        unsafe {
            TrackingState::from_ll(ll::ovrHmd_GetTrackingState(self.handle, abs_time))
        }
    }

    pub fn get_fov_texture_size(&self,
                                eye: Eye,
                                fov: FovPort,
                                pixels_per_display_pixel: f32) -> ll::Sizei {
        unsafe {
            ll::ovrHmd_GetFovTextureSize(self.handle,
                                         eye.to_ll(),
                                         fov.to_ll(),
                                         pixels_per_display_pixel)
        }
    }

    pub fn configure_rendering<RC: ToRenderConfig>(&self,
                               api_config: &RC,
                               cap: DistortionCapabilities,
                               eye_fov: PerEye<FovPort>) -> Option<PerEye<EyeRenderDescriptor>> {
        unsafe {
            let mut out: PerEye<ll::EyeRenderDesc> = PerEye::new(Default::default(),
                                                                 Default::default());
            let was_started = ll::ovrHmd_ConfigureRendering(
                self.handle,
                &api_config.to_render_config(),
                cap.flags,
                eye_fov.map(|_, d| d.to_ll()).ptr(),
                out.mut_ptr()
            );

            if was_started {
                Some(out.map(|_, d| EyeRenderDescriptor::from_ll(d)))
            } else {
                None
            }
        }
    }

    pub fn begin_frame(&self, frame_index: usize) -> FrameTiming {
        unsafe {
            FrameTiming::from_ll(
                ll::ovrHmd_BeginFrame(self.handle, frame_index as c_uint)
            )
        }
    }

    pub fn end_frame(&self) {
        unsafe {
            ll::ovrHmd_EndFrame(self.handle);
        }
    }

    pub fn begin_eye_render(&self, eye: Eye) -> Pose {
        unsafe {
            Pose::from_ll(ll::ovrHmd_BeginEyeRender(self.handle, eye.to_ll()))
        }
    }

    pub fn end_eye_render<T: ToTexture>(&self,
                                        eye: Eye,
                                        pose: Pose,
                                        texture: &T) {
        unsafe {
            ll::ovrHmd_EndEyeRender(self.handle,
                                    eye.to_ll(),
                                    pose.to_ll(),
                                    &texture.to_texture());
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HmdCapabilities {
    flags: c_uint
}

impl HmdCapabilities {
    pub fn present(&self) -> bool {
        self.flags & ll::HmdCap_Present == ll::HmdCap_Present
    }

    pub fn available(&self) -> bool {
        self.flags & ll::HmdCap_Available == ll::HmdCap_Available
    }

    pub fn low_persistance(&self) -> bool {
        self.flags & ll::HmdCap_LowPersistence 
                == ll::HmdCap_LowPersistence
    }

    pub fn set_low_persistance(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_LowPersistence
            } else {
                self.flags & !ll::HmdCap_LowPersistence
            }
        }
    }

    pub fn latency_test(&self) -> bool {
        self.flags & ll::HmdCap_LatencyTest == ll::HmdCap_LatencyTest
    }

    pub fn set_latency_test(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_LatencyTest
            } else {
                self.flags & !ll::HmdCap_LatencyTest
            }
        }
    }

    pub fn dynamic_prediction(&self) -> bool {
        self.flags & ll::HmdCap_DynamicPrediction 
                == ll::HmdCap_DynamicPrediction
    }

    pub fn set_dynamic_prediction(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_DynamicPrediction
            } else {
                self.flags & !ll::HmdCap_DynamicPrediction
            }
        }
    }

    pub fn no_vsync(&self) -> bool {
        self.flags & ll::HmdCap_NoVSync == ll::HmdCap_NoVSync
    }

    pub fn set_no_vsync(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_NoVSync
            } else {
                self.flags & !ll::HmdCap_NoVSync
            }
        }
    }

    pub fn no_restore(&self) -> bool {
        self.flags & ll::HmdCap_NoRestore == ll::HmdCap_NoRestore
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TrackingCapabilities {
    flags: c_uint
}

impl TrackingCapabilities {
    pub fn new() -> TrackingCapabilities {
        TrackingCapabilities {
            flags: 0
        }
    }

    pub fn orientation(&self) -> bool {
        self.flags & ll::TrackingCap_Orientation ==
            ll::TrackingCap_Orientation
    }

    pub fn yaw_correction(&self) -> bool {
        self.flags & ll::TrackingCap_MagYawCorrection ==
            ll::TrackingCap_MagYawCorrection
    }

    pub fn position(&self) -> bool {
        self.flags & ll::TrackingCap_Position ==
            ll::TrackingCap_Position
    }

    pub fn idle(&self) -> bool {
        self.flags & ll::TrackingCap_Idle ==
            ll::TrackingCap_Idle
    }

    pub fn set_orientation(&self, flag: bool) -> TrackingCapabilities {
        TrackingCapabilities {flags: 
            if flag {
                self.flags | ll::TrackingCap_Orientation
            } else {
                self.flags & !ll::TrackingCap_Orientation
            }
        }
    }

    pub fn set_yaw_correction(&self, flag: bool) -> TrackingCapabilities {
        TrackingCapabilities {flags: 
            if flag {
                self.flags | ll::TrackingCap_MagYawCorrection
            } else {
                self.flags & !ll::TrackingCap_MagYawCorrection
            }
        }
    }

    pub fn set_position(&self, flag: bool) -> TrackingCapabilities {
        TrackingCapabilities {flags: 
            if flag {
                self.flags | ll::TrackingCap_Position
            } else {
                self.flags & !ll::TrackingCap_Position
            }
        }
    }

    pub fn set_idle(&self, flag: bool) -> TrackingCapabilities {
        TrackingCapabilities {flags:
            if flag {
                self.flags | ll::TrackingCap_Idle
            } else {
                self.flags & !ll::TrackingCap_Idle
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DistortionCapabilities {
    flags: c_uint
}

impl DistortionCapabilities {
    pub fn new() -> DistortionCapabilities {
        DistortionCapabilities {
            flags: 0
        }
    }

    pub fn chromatic(&self) -> bool {
        self.flags & ll::DistortionCap_Chromatic ==
            ll::DistortionCap_Chromatic
    }

    pub fn timewarp(&self) -> bool {
        self.flags & ll::DistortionCap_TimeWarp ==
            ll::DistortionCap_TimeWarp
    }

    pub fn vignette(&self) -> bool {
        self.flags & ll::DistortionCap_Vignette ==
            ll::DistortionCap_Vignette
    }

    pub fn set_chromatic(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_Chromatic
            } else {
                self.flags & !ll::DistortionCap_Chromatic
            }
        }
    }

    pub fn set_timewarp(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_TimeWarp
            } else {
                self.flags & !ll::DistortionCap_TimeWarp
            }
        }
    }

    pub fn set_vignette(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_Vignette
            } else {
                self.flags & !ll::DistortionCap_Vignette
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Status {
    flags: u32
}

impl Status {
    pub fn orientation_tracked(&self) -> bool {
        self.flags & ll::Status_OrientationTracked ==
            ll::Status_OrientationTracked
    }

    pub fn position_tracked(&self) -> bool {
        self.flags & ll::Status_PositionTracked ==
            ll::Status_PositionTracked
    }

    pub fn position_connected(&self) -> bool {
        self.flags & ll::Status_PositionConnected ==
            ll::Status_PositionConnected
    }

    pub fn hmd_connected(&self) -> bool {
        self.flags & ll::Status_HmdConnected ==
            ll::Status_HmdConnected
    }
}

fn to_quat(q: ll::Quaternionf) -> Quaternion<f32> {
    Quaternion::new(q.w, q.x, q.y, q.z)
}

fn to_vec3(v: ll::Vector3f) -> Vector3<f32> {
    Vector3::new(v.x, v.y, v.z)
}

fn to_mat4(ll: ll::Matrix4f) -> Matrix4<f32> {
    Matrix4::new(
        ll.m11, ll.m21, ll.m31, ll.m41,
        ll.m12, ll.m22, ll.m32, ll.m42,
        ll.m13, ll.m23, ll.m33, ll.m43,
        ll.m14, ll.m24, ll.m34, ll.m44
    )
}

fn from_quat(q: Quaternion<f32>) -> ll::Quaternionf {
    ll::Quaternionf {
        x: q.v.x, y: q.v.y, z: q.v.z, w: q.s
    }
}

fn from_vec3(v: Vector3<f32>) -> ll::Vector3f {
    ll::Vector3f {
        x: v.x, y: v.y, z: v.z
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pose {
    pub orientation: Quaternion<f32>,
    pub position: Vector3<f32>
}

impl Pose {
    fn from_ll(pose: ll::Posef) -> Pose {
        Pose {
            orientation: to_quat(pose.orientation),
            position: to_vec3(pose.position),
        }
    }

    fn to_ll(&self) -> ll::Posef {
        ll::Posef {
            orientation: from_quat(self.orientation),
            position: from_vec3(self.position),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PoseState {
    pub pose: Pose,
    pub angular_velocity: Vector3<f32>,
    pub linear_velocity: Vector3<f32>,
    pub angular_acceleration: Vector3<f32>,
    pub linear_acceleration: Vector3<f32>,
    pub time_in_seconds: f64
}

impl PoseState {
    fn from_ll(pose: ll::PoseState) -> PoseState {
        PoseState {
            pose: Pose::from_ll(pose.pose),
            angular_velocity: to_vec3(pose.angular_velocity),
            linear_velocity: to_vec3(pose.linear_velocity),
            angular_acceleration: to_vec3(pose.angular_acceleration),
            linear_acceleration: to_vec3(pose.linear_acceleration),
            time_in_seconds: pose.time_in_seconds as f64
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SensorData {
    pub accelerometer: Vector3<f32>,
    pub gyro: Vector3<f32>,
    pub magnetometer: Vector3<f32>,
    pub temperature: f64,
    pub time_in_seconds: f64
}

impl SensorData {
    fn from_ll(sensor_data: ll::SensorData) -> SensorData {
        SensorData {
            accelerometer: to_vec3(sensor_data.accelerometer),
            gyro: to_vec3(sensor_data.gyro),
            magnetometer: to_vec3(sensor_data.magnetometer),
            temperature: sensor_data.temperature as f64,
            time_in_seconds: sensor_data.time_in_seconds as f64
        }
    }
}

#[derive(Copy, Clone)]
pub struct TrackingState {
    pub head_pose: PoseState,
    pub camera_pose: Pose,
    pub leveled_camera_pose: Pose,
    pub raw_sensor_data: SensorData,
    pub status_flags: Status,
    pub last_vision_processing_time: f64,
    pub last_vision_frame_latency: f64,
    pub last_camera_frame_counter: u32,
}

impl TrackingState {
    fn from_ll(ts: ll::TrackingState) -> TrackingState {
        TrackingState {
            head_pose: PoseState::from_ll(ts.head_pose),
            camera_pose: Pose::from_ll(ts.camera_pose),
            leveled_camera_pose: Pose::from_ll(ts.leveled_camera_pose),
            raw_sensor_data: SensorData::from_ll(ts.raw_sensor_data),
            status_flags: Status{flags: ts.status_flags},
            last_vision_processing_time: ts.last_vision_processing_time as f64,
            last_vision_frame_latency: ts.last_vision_frame_latency as f64,
            last_camera_frame_counter: ts.last_camera_frame_counter as u32
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Eye {
    Left,
    Right
}

impl Eye {
    fn from_ll(c: c_uint) -> Eye {
        match c {
            ll::Eye_Left => Eye::Left,
            ll::Eye_Right => Eye::Right,
            _ => panic!("Invalid eye type {}", c)
        }
    }

    fn to_ll(&self) -> c_uint {
        match *self {
            Eye::Left => ll::Eye_Left,
            Eye::Right => ll::Eye_Right
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PerEye<T> {
    pub left: T,
    pub right: T
}

impl<T> PerEye<T> {
    pub fn new(left: T, right: T) -> PerEye<T> {
        PerEye {
            left: left,
            right: right
        }
    }

    pub fn eye<'a>(&'a self, eye: Eye) -> &'a T {
        match eye {
            Eye::Left => &self.left,
            Eye::Right => &self.right
        }
    }

    pub fn map<U, F>(&self, mut f: F) -> PerEye<U> where F: FnMut(Eye, &T) -> U {
        PerEye::new(
            f(Eye::Left, &self.left),
            f(Eye::Right, &self.right)
        )
    }

    pub unsafe fn ptr(&self) -> *const T {
        &self.left as *const T
    }

    pub unsafe fn mut_ptr(&mut self) -> *mut T {
        &mut self.left as *mut T
    }
}

fn from_buf(ptr: *const u8) -> String {
    use std::ffi::{CString, c_str_to_bytes};
    unsafe { CString::from_slice(c_str_to_bytes(&(ptr as *const i8)))
            .container_as_str()
            .unwrap()
            .to_string() }
}

impl Hmd {
}

#[derive(Debug, Copy, Clone)]
pub struct EyeRenderDescriptor {
    pub eye: Eye,
    pub fov: FovPort,
    pub distorted_viewport: ll::Recti,
    pub pixels_per_tan_angle_at_center: Vector2<f32>,
    pub view_adjust: Vector3<f32>
}

impl EyeRenderDescriptor {
    fn from_ll(d: &ll::EyeRenderDesc) -> EyeRenderDescriptor {
        EyeRenderDescriptor {
            eye: Eye::from_ll(d.eye),
            fov: FovPort::from_ll(d.fov),
            distorted_viewport: d.distorted_viewport,
            pixels_per_tan_angle_at_center:
                Vector2::new(d.pixels_per_tan_angle_at_center.x,
                             d.pixels_per_tan_angle_at_center.y),
            view_adjust: Vector3::new(d.view_adjust.x,
                                      d.view_adjust.y,
                                      d.view_adjust.z)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RenderGLConfig {
    pub size: ll::Sizei,
    pub multisample: isize,
    pub display: Option<*const c_void>,
    pub window: Option<*const c_void>
}

pub trait ToRenderConfig {
    fn to_render_config(&self) -> ll::RenderApiConfig;
}

impl ToRenderConfig for RenderGLConfig {
    fn to_render_config(&self) -> ll::RenderApiConfig {
        ll::RenderApiConfig {
            header: ll::RenderApiConfigHeader {
                render_api_type: ll::RenderAPI_OpenGL,
                rt_size: self.size,
                multisample: self.multisample as c_int,
            },
            display: match self.display { Some(p) => p, None => ptr::null() },
            window: match self.window { Some(p) => p, None => ptr::null() },
            padd: [ptr::null(), ptr::null(), ptr::null(),
                   ptr::null(), ptr::null(), ptr::null()]
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FrameTiming {
    pub delta_seconds: f32,
    pub this_frame_seconds: f64,
    pub timewarp_point_seconds: f64,
    pub next_frame_seconds: f64,
    pub scanout_midpoint_seconds: f64,
    pub eye_scanout_seconds: PerEye<f64>
}

impl FrameTiming {
    fn from_ll(old: ll::FrameTiming) -> FrameTiming {
        FrameTiming {
            delta_seconds: old.delta_seconds,
            this_frame_seconds: old.this_frame_seconds,
            timewarp_point_seconds: old.timewarp_point_seconds,
            next_frame_seconds: old.next_frame_seconds,
            scanout_midpoint_seconds: old.scanout_midpoint_seconds,
            eye_scanout_seconds: PerEye::new(old.eye_scanout_seconds[ll::Eye_Left as usize],
                                             old.eye_scanout_seconds[ll::Eye_Right as usize])
        }
    }
}

pub trait ToTexture {
    fn to_texture(&self) -> ll::Texture;
}

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub size: ll::Sizei,
    pub viewport: ll::Recti,
    pub texture: u32
}

impl Texture {
    pub fn new(width: isize,
               height: isize,
               viewport_x: isize,
               viewport_y: isize,
               viewport_width: isize,
               viewport_height: isize,
               opengl_texture: u32) -> Texture {
        Texture {
            size: ll::Sizei {
                x: width as i32,
                y: height as i32
            },
            viewport: ll::Recti {
                pos: ll::Vector2i {
                    x: viewport_x as i32,
                    y: viewport_y as i32
                },
                size: ll::Sizei {
                    x: viewport_width as i32,
                    y: viewport_height as i32
                }
            },
            texture: opengl_texture
        }
    }
}

impl ToTexture for Texture {
    fn to_texture(&self) -> ll::Texture {
        ll::Texture {
            header: ll::TextureHeader {
                render_api_type: ll::RenderAPI_OpenGL,
                size: self.size,
                viewport: self.viewport,
            },
            texture_id: self.texture,
            padd: [ptr::null(), ptr::null(), ptr::null(), ptr::null(),
                   ptr::null(), ptr::null(), ptr::null()]
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FovPort {
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32
}

impl FovPort {
    fn from_ll(ll: ll::FovPort) -> FovPort {
        FovPort {
            up: ll.up_tan as f32,
            down: ll.down_tan as f32,
            left: ll.left_tan as f32,
            right: ll.right_tan as f32
        }
    }

    fn to_ll(&self) -> ll::FovPort {
        ll::FovPort {
            up_tan: self.up as c_float,
            down_tan: self.down as c_float,
            left_tan: self.left as c_float,
            right_tan: self.right as c_float
        }
    }

    pub fn projection(&self, znear: f32, zfar: f32, right_handed: bool) -> Matrix4<f32> {
        unsafe {
            let mat = ll::ovrMatrix4f_Projection(self.to_ll(), znear, zfar, right_handed);
            to_mat4(mat)
        }
    }
}
