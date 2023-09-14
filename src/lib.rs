//! A [macroquad](https://macroquad.rs/) plugin to access connected gamepads in the browser.
//!
//! There is only one [Gamepads::list] entrypoint function exposed to retrieve information about gamepads.
//!
//! TODO: Practical usage instructions.
//!
//! ```
//! let gamepads = Gamepads::new();
//!
//! loop {
//!     for gamepad in gamepads.list() {
//!         for button in gamepad.buttons() {
//!         }
//!     }
//!     // ...
//! }
//! ```
//!
//!
//! TODO: Give overview of specification and browser quirks?
//!
//! Resources:
//! * [MDN documentation about the Gamepad API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
//! * [Gamepad W3C specification](https://w3c.github.io/gamepad/)
//! * [Gamepad Extensions W3C specification](https://w3c.github.io/gamepad/extensions.html)

/// An individual gamepad or other controller, allowing access to information such as button presses, axis positions,
/// and id.
///
/// A Gamepad object can be returned by grabbing any position in the array returned by the [get_gamepads()] method.
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad), the
/// [Gamepad specification](https://w3c.github.io/gamepad/#gamepad-interface), and the supplemental
/// [Gamepad Extensions specification](https://w3c.github.io/gamepad/extensions.html#partial-gamepad-interface).
///
#[derive(Debug)]
pub struct Gamepad<'a> {
    #[cfg(target_family = "wasm")]
    bytes: &'a [u8],
    #[cfg(target_family = "wasm")]
    last_pressed_bits: u32,
    #[cfg(not(target_family = "wasm"))]
    gilrs_gamepad: gilrs::Gamepad<'a>,
    #[cfg(not(target_family = "wasm"))]
    pressed_bits: u32,
    #[cfg(not(target_family = "wasm"))]
    just_pressed_bits: u32,
}

/// The different type of buttons.
///
/// ![Visual representation of a Standard Gamepad layout](https://w3c.github.io/gamepad/standard_gamepad.svg)
#[derive(Copy, Clone, Debug)]
pub enum ButtonType {
    /// 0. Bottom button in right cluster
    BottomRightCluster,
    /// 1. Right button in right cluster
    RightRightCluster,
    /// 2. Left button in right cluster
    LeftRightCluster,
    /// 3. Top button in right cluster
    TopRightCluster,
    /// 4. Top left front button
    TopLeftFront,
    /// 5. Top right front button
    TopRightFront,
    /// 6. Bottom left front button
    BottomLeftFront,
    /// 7. Bottom right front button
    BottomRightFront,
    /// 8. Left button in center cluster - select/back
    LeftCenterCluster,
    /// 9. Right button in center cluster - start/forward
    RightCenterCluster,
    /// 10. Left stick pressed button
    LeftStick,
    /// 11. Right stick pressed button
    RightStick,
    /// 12. Top button in left cluster
    TopLeftCluster,
    /// 13. Bottom button in left cluster
    BottomLeftCluster,
    /// 14. Left button in left cluster
    LeftLeftCluster,
    /// 15. Right button in left cluster
    RightLeftCluster,
    /// 16. Center button in center cluster
    CenterCenterCluster,
}

impl ButtonType {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::BottomRightCluster,
            Self::RightRightCluster,
            Self::LeftRightCluster,
            Self::TopRightCluster,
            Self::TopLeftFront,
            Self::TopRightFront,
            Self::BottomLeftFront,
            Self::BottomRightFront,
            Self::LeftCenterCluster,
            Self::RightCenterCluster,
            Self::LeftStick,
            Self::RightStick,
            Self::TopLeftCluster,
            Self::BottomLeftCluster,
            Self::LeftLeftCluster,
            Self::RightLeftCluster,
            Self::CenterCenterCluster,
        ]
        .into_iter()
    }

    // Compare https://docs.rs/gilrs/latest/gilrs/#controller-layout
    // and https://www.w3.org/TR/gamepad/#fig-visual-representation-of-a-standard-gamepad-layout
    /*
    #[cfg(not(target_family = "wasm"))]
    fn as_gilrs(self) -> gilrs::Button {
        match self {
            // Right cluster / Action pad
            Self::BottomRightCluster => gilrs::Button::South,
            Self::RightRightCluster => gilrs::Button::East,
            Self::LeftRightCluster => gilrs::Button::West,
            Self::TopRightCluster => gilrs::Button::North,
            // Top buttons
            Self::TopLeftFront => gilrs::Button::LeftTrigger,
            Self::TopRightFront => gilrs::Button::RightTrigger,
            Self::BottomLeftFront => gilrs::Button::LeftTrigger2,
            Self::BottomRightFront => gilrs::Button::RightTrigger2,
            // Center (start/select)
            Self::LeftCenterCluster => gilrs::Button::Select,
            Self::RightCenterCluster => gilrs::Button::Start,
            // Sticks pressed.
            Self::LeftStick => gilrs::Button::LeftThumb,
            Self::RightStick => gilrs::Button::RightThumb,
            // Left cluster / d-pad
            Self::TopLeftCluster => gilrs::Button::DPadUp,
            Self::BottomLeftCluster => gilrs::Button::DPadDown,
            Self::LeftLeftCluster => gilrs::Button::DPadLeft,
            Self::RightLeftCluster => gilrs::Button::DPadRight,
            // Center button?
            Self::CenterCenterCluster => gilrs::Button::Mode,
        }
    }
    */

    #[cfg(not(target_family = "wasm"))]
    fn from_gilrs(button: gilrs::Button) -> Option<Self> {
        Some(match button {
            // Right cluster / Action pad
            gilrs::Button::South => Self::BottomRightCluster,
            gilrs::Button::East => Self::RightRightCluster,
            gilrs::Button::West => Self::LeftRightCluster,
            gilrs::Button::North => Self::TopRightCluster,
            // Top buttons
            gilrs::Button::LeftTrigger => Self::TopLeftFront,
            gilrs::Button::RightTrigger => Self::TopRightFront,
            gilrs::Button::LeftTrigger2 => Self::BottomLeftFront,
            gilrs::Button::RightTrigger2 => Self::BottomRightFront,
            // Center (start/select)
            gilrs::Button::Select => Self::LeftCenterCluster,
            gilrs::Button::Start => Self::RightCenterCluster,
            // Sticks pressed.
            gilrs::Button::LeftThumb => Self::LeftStick,
            gilrs::Button::RightThumb => Self::RightStick,
            // Left cluster / d-pad
            gilrs::Button::DPadUp => Self::TopLeftCluster,
            gilrs::Button::DPadDown => Self::BottomLeftCluster,
            gilrs::Button::DPadLeft => Self::LeftLeftCluster,
            gilrs::Button::DPadRight => Self::RightLeftCluster,
            // Center button?
            gilrs::Button::Mode => Self::CenterCenterCluster,
            // Other:
            _ => {
                return None;
            }
        })
    }
}

/// Defines an individual button of a gamepad or other controller, allowing access to the current state of different
/// types of buttons available on the control device.
///
/// A `GamepadButton` object is returned by querying any value of the array returned by the [Gamepad::buttons] property
/// of the Gamepad interface.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GamepadButton {
    /// A boolean indicating whether the button is currently pressed (`true`) or unpressed (`false`).
    pub pressed: bool,

    /// A boolean indicating whether a button capable of detecting touch is currently touched (`true`) or not
    /// touched (`false`).
    ///
    /// If the button is not capable of detecting touch but can return an analog value, the property will be true if
    /// the value is greater than `0`, and false otherwise. If the button is not capable of detecting touch and can
    /// only report a digital value, then it should mirror the [Self::pressed] property.
    pub touched: bool,

    /// The current state of analog buttons on many modern gamepads, such as the triggers.
    ///
    /// Values are normalized to the range `0.0` — `1.0`, with `0.0` representing a button that is not pressed, and
    /// `1.0` representing a button that is fully pressed.
    pub value: f32,
}

/// What hand the controller is being held in, or is most likely to be held in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadHand {
    /// The left hand.
    Left,
    /// If the neither left or right is applicable - the controller is held in both hands or would be fine in either.
    None,
}

/// Hardware in the controller designed to provide haptic feedback to the user, most commonly vibration hardware.
///
/// This interface is accessible through the [Gamepad::hapticActuators] property.
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator) and the
/// [Gamepad Extensions specification](https://w3c.github.io/gamepad/extensions.html#dom-gamepadhapticactuator).
#[derive(Debug)]
pub struct GamepadHapticActuator {}

impl GamepadHapticActuator {
    /// Makes the hardware pulse at a certain intensity for a specified duration.
    /// TODO: duration type
    ///
    /// # Arguments
    ///
    /// * `value` - The intensity of the pulse. This can vary depending on the hardware type, but generally takes a
    ///   value between `0.0` (no intensity) and `1.0` (full intensity).
    /// * `duration` - The duration of the pulse in milliseconds.
    pub fn pulse(_value: f64, _duration: u32) {}
}

extern "C" {
    /// Provided by the miniquad-gamepads.js plugin.
    #[cfg(target_family = "wasm")]
    fn getGamepads(data_ptr: *const u8, max_length: usize) -> i32;

    /// Log function provided by miniquad in gl.js.
    #[cfg(target_family = "wasm")]
    fn console_log(msg: *const ::std::os::raw::c_char);
}

/// Debug log function.
pub fn log(output: &str) {
    #[cfg(target_family = "wasm")]
    {
        use std::ffi::CString;
        let c_string = CString::new(output).unwrap();
        unsafe {
            console_log(c_string.as_ptr());
        }
    }
    #[cfg(not(target_family = "wasm"))]
    {
        println!("{}", output);
    }
}

/// Expected by gl.js in miniquad.
#[cfg(target_family = "wasm")]
#[no_mangle]
pub extern "C" fn gamepads_crate_version() -> u32 {
    let major_version = 0; // (crate_version >> 24) & 0xff;
    let minor_version = 1; // (crate_version >> 16) & 0xff;
    let patch_version = 0; // crate_version & 0xffff;
    (major_version << 24) + (minor_version << 16) + patch_version
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GamepadId(usize);

impl<'a> Gamepad<'a> {
    #[cfg(target_family = "wasm")]
    const NUM_AXES: usize = 4;
    //const NUM_BUTTONS: usize = 17;

    /// An integer that is auto-incremented to be unique for each device currently connected to the system
    ///
    /// This can be used to distinguish multiple controllers; a gamepad that is disconnected and reconnected will
    /// retain the same index.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index) and the
    /// [Gamepad specification](https://w3c.github.io/gamepad/#dom-gamepad-index).
    pub fn id(&self) -> GamepadId {
        #[cfg(target_family = "wasm")]
        {
            GamepadId(self.bytes[0] as usize)
        }
        #[cfg(not(target_family = "wasm"))]
        {
            GamepadId(self.gilrs_gamepad.id().into())
        }
    }

    /// An array representing the controls with axes present on the device (e.g. analog thumb sticks).
    ///
    /// Each entry in the array is a floating point value in the range -1.0 – 1.0, representing the axis position from
    /// the lowest value (-1.0) to the highest value (1.0).
    ///
    /// If the controller is perpendicular to the ground with the directional stick pointing up, -1.0 SHOULD correspond
    /// to "forward" or "left", and 1.0 SHOULD correspond to "backward" or "right".
    /// Axes that are drawn from a 2D input device SHOULD appear next to each other in the axes array, X then Y. It is
    /// RECOMMENDED that axes appear in decreasing order of importance, such that element 0 and 1 typically represent
    /// the X and Y axis of a directional stick.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes) and the
    /// [Gamepad specification](https://w3c.github.io/gamepad/#dom-gamepad-axes).
    pub fn axes(&self) -> [f32; 4] {
        #[cfg(target_family = "wasm")]
        {
            let bytes_offset = 4;
            let bytes_length = Self::NUM_AXES * std::mem::size_of::<f32>();
            let bytes_slice = &self.bytes[bytes_offset..(bytes_offset + bytes_length)];
            let bytes_ptr = bytes_slice.as_ptr() as *const f32;
            let slice = unsafe { std::slice::from_raw_parts(bytes_ptr, Self::NUM_AXES) };
            slice.try_into().unwrap()
        }
        #[cfg(not(target_family = "wasm"))]
        {
            let a = self
                .gilrs_gamepad
                .axis_data(gilrs::Axis::LeftStickX)
                .map(gilrs::ev::state::AxisData::value)
                .unwrap_or_default();
            let b = -self
                .gilrs_gamepad
                .axis_data(gilrs::Axis::LeftStickY)
                .map(gilrs::ev::state::AxisData::value)
                .unwrap_or_default();
            let c = self
                .gilrs_gamepad
                .axis_data(gilrs::Axis::RightStickX)
                .map(gilrs::ev::state::AxisData::value)
                .unwrap_or_default();
            let d = -self
                .gilrs_gamepad
                .axis_data(gilrs::Axis::RightStickY)
                .map(gilrs::ev::state::AxisData::value)
                .unwrap_or_default();
            [a, b, c, d]
        }
    }

    /// Objects representing the buttons present on the device.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons) and the
    /// [Gamepad specification](https://w3c.github.io/gamepad/#dom-gamepad-buttons).
    pub fn buttons(&self) -> impl Iterator<Item = bool> + '_ {
        #[cfg(target_family = "wasm")]
        {
            let bytes_offset = 4 + Self::NUM_AXES * std::mem::size_of::<f32>();
            let pressed_bits = u32::from_le_bytes([
                self.bytes[bytes_offset],
                self.bytes[bytes_offset + 1],
                self.bytes[bytes_offset + 2],
                self.bytes[bytes_offset + 3],
            ]);
            let mut count = 0;
            std::iter::from_fn(move || {
                let result = if count == 17 {
                    None
                } else {
                    Some((pressed_bits & (1 << count)) != 0)
                };
                count += 1;
                result
            })
        }
        #[cfg(not(target_family = "wasm"))]
        {
            ButtonType::all().map(|t| self.is_pressed(t))
        }
    }

    pub fn just_pressed(&self, button_type: ButtonType) -> bool {
        #[cfg(target_family = "wasm")]
        {
            // TODO: JUST pressed
            let bytes_offset = 4 + Self::NUM_AXES * std::mem::size_of::<f32>();
            let pressed_bits = u32::from_le_bytes([
                self.bytes[bytes_offset],
                self.bytes[bytes_offset + 1],
                self.bytes[bytes_offset + 2],
                self.bytes[bytes_offset + 3],
            ]);
            // Just pressed positioned directly after:
            let queried_bit = 1 << (button_type as u32);
            (pressed_bits & queried_bit) != 0 && (self.last_pressed_bits & queried_bit) == 0
        }
        #[cfg(not(target_family = "wasm"))]
        {
            (self.just_pressed_bits & (1 << (button_type as u32))) != 0
        }
    }

    pub fn is_pressed(&self, button_type: ButtonType) -> bool {
        #[cfg(target_family = "wasm")]
        {
            let bytes_offset = 4 + Self::NUM_AXES * std::mem::size_of::<f32>();
            let pressed_bits = u32::from_le_bytes([
                self.bytes[bytes_offset],
                self.bytes[bytes_offset + 1],
                self.bytes[bytes_offset + 2],
                self.bytes[bytes_offset + 3],
            ]);
            (pressed_bits & (1 << (button_type as u32))) != 0
        }
        #[cfg(not(target_family = "wasm"))]
        {
            (self.pressed_bits & (1 << (button_type as u32))) != 0
        }
    }

    /// A boolean indicating whether the gamepad is still connected to the system.
    ///
    /// If the gamepad is connected, the value is `true`; if not, it is `false`.
    ///
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected) and the
    /// [Gamepad specification](https://w3c.github.io/gamepad/#dom-gamepad-connected).
    pub fn connected(&self) -> bool {
        #[cfg(target_family = "wasm")]
        {
            self.bytes[1] == 1
        }
        #[cfg(not(target_family = "wasm"))]
        {
            self.gilrs_gamepad.is_connected()
        }
    }

    //pub haptic_actuators: Vec<GamepadHapticActuator>,
}

const MAX_GAMEPADS: usize = 8;

/// Context for obtaining gamepad information.
///
/// Construct an instance using [Gamepads::new], then use [Gamepads::list] to list gamepads.
pub struct Gamepads {
    #[cfg(target_family = "wasm")]
    buffer: Vec<u8>,
    #[cfg(target_family = "wasm")]
    last_pressed_bits: [u32; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    gilrs_instance: gilrs::Gilrs,
    #[cfg(not(target_family = "wasm"))]
    num_gamepads: u8,
    #[cfg(not(target_family = "wasm"))]
    gamepad_ids: [usize; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    pressed_bits: [u32; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    just_pressed: [u32; MAX_GAMEPADS],
}

impl Gamepads {
    // NOTE: Must match what javascript encoder uses.
    #[cfg(target_family = "wasm")]
    const BYTES_PER_GAMEPAD: usize = 256;
    #[cfg(target_family = "wasm")]
    const BUFFER_LENGTH: usize = Self::BYTES_PER_GAMEPAD * MAX_GAMEPADS;

    /// Construct a new gamepads instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        assert_eq!(std::mem::size_of::<GamepadButton>(), 8);
        Self {
            #[cfg(target_family = "wasm")]
            buffer: vec![0; Self::BUFFER_LENGTH],
            #[cfg(target_family = "wasm")]
            last_pressed_bits: [0; MAX_GAMEPADS],
            #[cfg(not(target_family = "wasm"))]
            gilrs_instance: gilrs::Gilrs::new().unwrap(),
            #[cfg(not(target_family = "wasm"))]
            num_gamepads: 0,
            #[cfg(not(target_family = "wasm"))]
            pressed_bits: [0; MAX_GAMEPADS],
            #[cfg(not(target_family = "wasm"))]
            just_pressed: [0; MAX_GAMEPADS],
            #[cfg(not(target_family = "wasm"))]
            gamepad_ids: [0; MAX_GAMEPADS],
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn find_or_insert(&mut self, gilrs_gamepad_id: gilrs::GamepadId) -> Option<usize> {
        for i in 0..(self.num_gamepads as usize) {
            if self.gamepad_ids[i] == gilrs_gamepad_id.into() {
                return Some(i);
            }
        }
        if self.num_gamepads as usize == MAX_GAMEPADS {
            None
        } else {
            let index = self.num_gamepads as usize;
            self.num_gamepads += 1;
            self.gamepad_ids[index] = gilrs_gamepad_id.into();
            Some(index)
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn try_find(&self, gilrs_gamepad_id: gilrs::GamepadId) -> Option<usize> {
        (0..(self.num_gamepads as usize)).find(|&i| self.gamepad_ids[i] == gilrs_gamepad_id.into())
    }

    /// List information about gamepads.
    pub fn list(&mut self) -> impl Iterator<Item = Gamepad> {
        #[cfg(target_family = "wasm")]
        {
            let pointer = self.buffer.as_ptr();
            let returned_length = unsafe { getGamepads(pointer, Self::BUFFER_LENGTH) } as usize;

            let buffer = &self.buffer;
            let last_pressed_bits = &self.last_pressed_bits;
            let mut count = 0;
            std::iter::from_fn(move || {
                let result = if count == returned_length {
                    None
                } else {
                    let bytes_offset = Self::BYTES_PER_GAMEPAD * count;
                    let bytes_end = bytes_offset + Self::BYTES_PER_GAMEPAD;
                    Some(Gamepad {
                        bytes: &buffer[bytes_offset..bytes_end],
                        last_pressed_bits: last_pressed_bits[count],
                    })
                };
                count += 1;
                result
            })
        }
        #[cfg(not(target_family = "wasm"))]
        {
            while let Some(gilrs::Event { id, event, .. }) = self.gilrs_instance.next_event() {
                if let Some(gamepad_idx) = self.find_or_insert(id) {
                    match event {
                        gilrs::EventType::ButtonPressed(button, _code) => {
                            if let Some(b) = ButtonType::from_gilrs(button) {
                                let bit = 1 << (b as u32);
                                self.pressed_bits[gamepad_idx] |= bit;
                                self.just_pressed[gamepad_idx] |= bit;
                            }
                        }
                        gilrs::EventType::ButtonReleased(button, _code) => {
                            if let Some(b) = ButtonType::from_gilrs(button) {
                                let bit = 1 << (b as u32);
                                self.pressed_bits[gamepad_idx] &= !bit;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let result =
                self.gilrs_instance
                    .gamepads()
                    .filter_map(|(gilrs_gamepad_id, gilrs_gamepad)| {
                        if let Some(gamepad_idx) = self.try_find(gilrs_gamepad_id) {
                            Some(Gamepad {
                                gilrs_gamepad,
                                pressed_bits: self.pressed_bits[gamepad_idx],
                                just_pressed_bits: self.just_pressed[gamepad_idx],
                            })
                        } else {
                            None
                        }
                    });

            result
        }
    }

    pub fn clear(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        {
            self.just_pressed.fill(0);
        }
        #[cfg(target_family = "wasm")]
        {
            for i in 0..self::MAX_GAMEPADS {
                let bytes_offset = i * Self::BYTES_PER_GAMEPAD
                    + 4
                    + Gamepad::NUM_AXES * std::mem::size_of::<f32>();
                let pressed_bits = u32::from_le_bytes([
                    self.buffer[bytes_offset],
                    self.buffer[bytes_offset + 1],
                    self.buffer[bytes_offset + 2],
                    self.buffer[bytes_offset + 3],
                ]);
                self.last_pressed_bits[i] = pressed_bits;
            }
        }
    }
}
