use crate::Gamepad;

extern "C" {
    // Host javascript function.
    pub fn getGamepads(data_ptr: *const Gamepad);

    // Host javascript function.
    pub fn playEffect(
        gamepad_id: u8,
        duration_ms: u32,
        start_delay_ms: u32,
        strong_magnitude: f32,
        weak_magnitude: f32,
    );
}

/// Expose crate version information as expected by
/// https://github.com/not-fl3/miniquad/blob/master/js/gl.js.
#[cfg(target_family = "wasm")]
#[no_mangle]
pub extern "C" fn gamepads_crate_version() -> u32 {
    let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    (major << 24) + (minor << 16) + patch
}
