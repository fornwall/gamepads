//! Library providing information about connected gamepads.
//!
//! # Overview
//! To use `gamepads`, first create a [Gamepads] instance using [Gamepads::new()].
//!
//! Then, on each tick, run [Gamepads::poll()] to poll gamepad state, followed by
//! [Gamepads::all()] or [Gamepads::get()] to retrieve information about connected
//! gamepads. See [Gamepad] for how to access button and axis information on gamepads.
//!
//! # Usage
//! The `gamepads` crate is [on crates.io](https://crates.io/crates/gamepads) and can be
//! used by adding `gamepads` to your dependencies in your project's `Cargo.toml`.
//! Or more simply, just run `cargo add gamepads`.
//!
//! Here is a complete example that creates a new Rust project, adds a dependency
//! on `gamepads`, creates the source code printing gamepad information,  and then
//! runs the program.
//!
//! First, create the project in a new directory:
//!
//! ```sh
//! $ mkdir gamepads-example
//! $ cd gamepads-example
//! $ cargo init
//! ```
//!
//! Second, add a dependency on `gamepads`:
//!
//! ```sh
//! $ cargo add gamepads
//! ```
//!
//! Third, edit `src/main.rs`. Delete what's there and replace it with this:
//!
//! ```
//! use gamepads::Gamepads;
//!
//! fn main() {
//!     let mut gamepads = Gamepads::new();
//!     loop {
//!         # break;
//!         gamepads.poll();
//!
//!         for gamepad in gamepads.all() {
//!             println!("Gamepad id: {:?}", gamepad.id());
//!             for button in gamepad.all_currently_pressed() {
//!                 println!("Pressed button: {:?}", button);
//!             }
//!             println!("Left thumbstick: {:?}", gamepad.left_stick());
//!             println!("Right thumbstick: {:?}", gamepad.right_stick());
//!         }
//!
//!         std::thread::sleep(std::time::Duration::from_millis(500));
//!    }
//! }
//! ```
//!
//! Fourth, run it with `cargo run`:
//!
//! ```sh
//! $ cargo run
//! [...]
//! Gamepad id: GamepadId(0)
//! Pressed button: ActionRight
//! Pressed button: ActionTop
//! Left thumbstick: (0.3252289, -0.98961794)
//! Right thumbstick: (0.0, 0.0)
//! ```
//!
//! # Usage as a macroquad web plugin
//! See the [documentation in the README](https://github.com/fornwall/gamepads#how-to-use-as-a-macroquad-plugin)
//! for how to use `gamepads` with `macroquad`.
//!
//! # Example showing gamepad iteration
//!
//! ```
//! use gamepads::{Button, Gamepads};
//!
//! let mut gamepads = Gamepads::new();
//!
//! loop {
//!     # break;
//!     gamepads.poll();
//!
//!     for gamepad in gamepads.all() {
//!         // Use just_pressed_buttons() or currently_pressed_buttons().
//!         for button in gamepad.all_just_pressed() {
//!             println!("Button just pressed: {button:?}");
//!             match button {
//!                 Button::DPadUp => println!("Going up!"),
//!                 Button::ActionDown => println!("Shooting!"),
//!                 _ => {}
//!             }
//!
//!             // Individual buttons can be checked using
//!             // is_just_pressed() / is_currently_pressed():
//!             if gamepad.is_currently_pressed(Button::FrontLeftLower) {
//!                 println!("Front left lower button is currently pressed");
//!             }
//!         }
//!         
//!         println!("Left stick: {:?}", gamepad.left_stick());
//!         println!("Right stick: {:?}", gamepad.right_stick());
//!     }
//! }
//! ```
//!
//! # Example showing gamepad lookup by id and haptic feedback
//!
//! ```
//! use std::collections::HashMap;
//! use gamepads::{Button, Gamepads, GamepadId};
//!
//! struct Player {
//!     position: (f32, f32),
//! }
//!
//! let mut gamepads = Gamepads::new();
//! let mut players: HashMap<GamepadId, Player> = HashMap::new();
//! loop {
//!     # break;
//!     gamepads.poll();
//!
//!     for gamepad in gamepads.all() {
//!         if let Some(player) = players.get_mut(&gamepad.id()) {
//!             player.position.0 += gamepad.left_stick_x();
//!             player.position.1 += gamepad.left_stick_y();
//!             if player.position.0.abs() > 10. {
//!                 // Player has fallen out of map - give haptic feedback.
//!                 gamepads.rumble(gamepad.id(), 500, 0, 0.4, 0.6);
//!                 player.position.0 = 0.;
//!             }
//!         } else if gamepad.is_currently_pressed(Button::ActionDown) {
//!             println!("New player joining with gamepad {:?}", gamepad.id());
//!             players.insert(gamepad.id(), Player { position: (0., 0.) });
//!         }
//!     }
//! }
//! ```

#[cfg(feature = "wasm-bindgen")]
mod wasm_bindgen;

const MAX_GAMEPADS: usize = 8;

/// An individual gamepad allowing access to information about button presses,
/// thumbstick positions and its gamepad id.
///
/// A gamepad can be obtained using either [Gamepads::all()] to loop through all connected gamepads,
/// or [Gamepads::get(gamepad_id)](Gamepads::get) to get it by an id.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Gamepad {
    id: GamepadId,
    connected: bool,
    pressed_bits: u32,
    axes: [f32; 4],
    #[cfg(target_family = "wasm")]
    last_pressed_bits: u32,
    #[cfg(not(target_family = "wasm"))]
    just_pressed_bits: u32,
}

// Assert size of struct Gamepad, which is used by javascript.
//
// See https://users.rust-lang.org/t/ensure-that-struct-t-has-size-n-at-compile-time/61108/3
#[cfg(target_family = "wasm")]
const _: () = [(); 1][(core::mem::size_of::<Gamepad>() == 28) as usize ^ 1];

impl Gamepad {
    /// An id unique for each gamepad currently connected to the system.
    ///
    /// This can be used to distinguish multiple controllers; a gamepad that is disconnected
    /// and reconnected will retain the same id.
    pub const fn id(&self) -> GamepadId {
        self.id
    }

    /// The `(x, y)` position of the left thumbstick.
    ///
    /// Each component is in the range `[-1.0, 1.0]`, with
    /// negative values representing down or to the left.
    pub const fn left_stick(&self) -> (f32, f32) {
        (self.axes[0], self.axes[1])
    }

    /// The `x` position of the left thumbstick.
    ///
    /// Values are in the range `[-1.0, 1.0]`, with
    /// negative values representing left.
    pub const fn left_stick_x(&self) -> f32 {
        self.axes[0]
    }

    /// The `y` position of the left thumbstick.
    ///
    /// Values are in the range `[-1.0, 1.0]`, with
    /// negative values representing down.
    pub const fn left_stick_y(&self) -> f32 {
        self.axes[1]
    }

    /// The `(x, y)` position of the right thumbstick.
    ///
    /// Each component is in the range `[-1.0, 1.0]`, with
    /// negative values representing down or to the left.
    pub const fn right_stick(&self) -> (f32, f32) {
        (self.axes[2], self.axes[3])
    }

    /// The `y` position of the right thumbstick.
    ///
    /// Values are in the range `[-1.0, 1.0]`, with
    /// negative values representing left.
    pub const fn right_stick_x(&self) -> f32 {
        self.axes[2]
    }

    /// The `y` position of the right thumbstick.
    ///
    /// Values are in the range `[-1.0, 1.0]`, with
    /// negative values representing down.
    pub const fn right_stick_y(&self) -> f32 {
        self.axes[3]
    }

    /// An iterator over all currently pressed buttons.
    pub fn all_currently_pressed(&self) -> impl Iterator<Item = Button> + '_ {
        Button::all().filter(|&t| self.is_currently_pressed(t))
    }

    /// An iterator over all just pressed buttons.
    pub fn all_just_pressed(&self) -> impl Iterator<Item = Button> + '_ {
        Button::all().filter(|&t| self.is_just_pressed(t))
    }

    /// Check if a button has just been pressed.
    pub const fn is_just_pressed(&self, button: Button) -> bool {
        let queried_bit = 1 << (button as u32);
        #[cfg(target_family = "wasm")]
        {
            (self.pressed_bits & queried_bit) != 0 && (self.last_pressed_bits & queried_bit) == 0
        }
        #[cfg(not(target_family = "wasm"))]
        {
            (self.just_pressed_bits & queried_bit) != 0
        }
    }

    /// Check if a button is currently pressed.
    pub const fn is_currently_pressed(&self, button: Button) -> bool {
        let queried_bit = 1 << (button as u32);
        (self.pressed_bits & queried_bit) != 0
    }
}

extern "C" {
    // Host javascript function.
    #[cfg(target_family = "wasm")]
    #[cfg(not(feature = "wasm-bindgen"))]
    fn getGamepads(data_ptr: *const Gamepad);

    // Host javascript function.
    #[cfg(target_family = "wasm")]
    #[cfg(not(feature = "wasm-bindgen"))]
    fn playEffect(
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

/// An opaque gamepad identifier.
///
/// Obtained using the [Gamepad::id()] method on a gamepad.
///
/// Given a gamepad id, it's possible to get its gamepad state using [Gamepads::get(gamepad_id)](Gamepads::get).
///
/// This is a small handle consisting of a single byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GamepadId(u8);

impl GamepadId {
    /// The byte value that represents this gamepad id.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// Context for obtaining gamepad information.
///
/// Construct an instance using [Gamepads::new].
///
/// On each tick, update gamepads state using [Gamepads::poll()].
///
/// Then use [Gamepads::all()] to list all connected gamepads, or [Gamepads::get(gamepad_id)](Gamepads::get)
/// to get a gamepad by id.
pub struct Gamepads {
    gamepads: [Gamepad; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    gilrs_gamepad_ids: [usize; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    gilrs_instance: gilrs::Gilrs,
    #[cfg(not(target_family = "wasm"))]
    num_connected_pads: u8,
    #[cfg(not(target_family = "wasm"))]
    deadzones: [[f32; 4]; MAX_GAMEPADS],
    #[cfg(not(target_family = "wasm"))]
    playing_ff_effects: Vec<(gilrs::ff::Effect, u128)>,
}

impl Gamepads {
    /// Construct a new gamepads instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let result = Self {
            gamepads: std::array::from_fn(|idx| Gamepad {
                id: GamepadId(idx as u8),
                connected: false,
                pressed_bits: 0,
                axes: [0.; 4],
                #[cfg(target_family = "wasm")]
                last_pressed_bits: 0,
                #[cfg(not(target_family = "wasm"))]
                just_pressed_bits: 0,
            }),
            #[cfg(not(target_family = "wasm"))]
            gilrs_gamepad_ids: [usize::MAX; MAX_GAMEPADS],
            #[cfg(not(target_family = "wasm"))]
            gilrs_instance: gilrs::Gilrs::new().unwrap(),
            #[cfg(not(target_family = "wasm"))]
            num_connected_pads: 0,
            #[cfg(not(target_family = "wasm"))]
            deadzones: [[0.; 4]; MAX_GAMEPADS],
            #[cfg(not(target_family = "wasm"))]
            playing_ff_effects: Vec::new(),
        };
        let mut result = result;
        result.poll();
        #[cfg(not(target_family = "wasm"))]
        {
            let gamepad_ids = result
                .gilrs_instance
                .gamepads()
                .map(|(id, g)| (id, g.is_connected()))
                .collect::<Vec<_>>();
            for (id, connected) in gamepad_ids {
                if let Some(p) = result.find_or_insert(id) {
                    result.gamepads[p].connected = connected;
                }
            }
        }
        result
    }

    #[cfg(not(target_family = "wasm"))]
    fn find_or_insert(&mut self, gilrs_gamepad_id: gilrs::GamepadId) -> Option<usize> {
        for i in 0..MAX_GAMEPADS {
            if self.gilrs_gamepad_ids[i] == gilrs_gamepad_id.into() {
                return Some(i);
            }
        }
        if self.num_connected_pads == MAX_GAMEPADS as u8 {
            None
        } else {
            let index = self.num_connected_pads;
            self.num_connected_pads += 1;
            self.gilrs_gamepad_ids[index as usize] = gilrs_gamepad_id.into();
            Some(index as usize)
        }
    }

    /// Get a gamepad by id, returning `None` if it is no longer connected.
    ///
    /// The gamepad state obtained here will reflect the state the last time [Gamepads::poll()]
    /// was called.
    pub fn get(&self, gamepad_id: GamepadId) -> Option<Gamepad> {
        let pad = self.gamepads[gamepad_id.0 as usize];
        pad.connected.then_some(pad)
    }

    /// Retrieve information about all connected gamepads.
    ///
    /// The gamepad state obtained here will reflect the state the last time [Gamepads::poll()]
    /// was called.
    pub fn all(&self) -> impl Iterator<Item = Gamepad> {
        self.gamepads.into_iter().filter(|p| p.connected)
    }

    /// Provide haptic feedback by rumbling the gamepad (if supported).
    ///
    /// This is a "dual rumble", where an eccentric rotating mass (ERM) vibration motor in each handle
    /// of the gamepad. Either motor is capable of vibrating the whole gamepad. The vibration effects
    /// created by each motor are unequal so that the effects of each can be combined to create more
    /// complex haptic effects.
    ///
    /// # Arguments
    ///
    /// * `gamepad_id` - ID of the gamepad to rumble
    /// * `duration_ms` - Duration of the rumble in milliseconds
    /// * `start_delay_ms` - Delay of the rumble in milliseconds
    /// * `strong_magnitude` - The vibration magnitude for the low frequency rumble in the range `[0.0, 1.0]`
    /// * `weak_magnitude` - The vibration magnitude for the high frequency rumble in the range `[0.0, 1.0]`
    pub fn rumble(
        &mut self,
        gamepad_id: GamepadId,
        duration_ms: u32,
        start_delay_ms: u32,
        strong_magnitude: f32,
        weak_magnitude: f32,
    ) {
        #[cfg(target_family = "wasm")]
        {
            #[cfg(not(feature = "wasm-bindgen"))]
            unsafe {
                playEffect(
                    gamepad_id.0,
                    duration_ms,
                    start_delay_ms,
                    strong_magnitude,
                    weak_magnitude,
                );
            }
            #[cfg(feature = "wasm-bindgen")]
            wasm_bindgen::play_effect(
                gamepad_id.0,
                duration_ms,
                start_delay_ms,
                strong_magnitude,
                weak_magnitude,
            );
        }
        #[cfg(not(target_family = "wasm"))]
        {
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // Purge old effects.
            for i in (0..self.playing_ff_effects.len()).rev() {
                if self.playing_ff_effects[i].1 < now_ms {
                    self.playing_ff_effects.swap_remove(i);
                }
            }

            let gilrs_gamepad_id = self.gilrs_gamepad_ids[gamepad_id.0 as usize];
            let gilrs_gamepad_id: gilrs::GamepadId =
                unsafe { std::mem::transmute(gilrs_gamepad_id) };

            let play_for = gilrs::ff::Ticks::from_ms(duration_ms);
            let after = gilrs::ff::Ticks::from_ms(start_delay_ms);
            let scheduling = gilrs::ff::Replay {
                play_for,
                after,
                ..Default::default()
            };

            let strong_magnitude = (f32::from(u16::MAX) * strong_magnitude).round() as u16;
            let weak_magnitude = (f32::from(u16::MAX) * weak_magnitude).round() as u16;

            if let Ok(effect) = gilrs::ff::EffectBuilder::new()
                .add_effect(gilrs::ff::BaseEffect {
                    kind: gilrs::ff::BaseEffectType::Strong {
                        magnitude: strong_magnitude,
                    },
                    scheduling,
                    ..Default::default()
                })
                .add_effect(gilrs::ff::BaseEffect {
                    kind: gilrs::ff::BaseEffectType::Weak {
                        magnitude: weak_magnitude,
                    },
                    scheduling,
                    ..Default::default()
                })
                .repeat(gilrs::ff::Repeat::For(play_for + after))
                .gamepads(&[gilrs_gamepad_id])
                .finish(&mut self.gilrs_instance)
            {
                if effect.play().is_ok() {
                    // Effects stop playing in drop(), so keep a reference.
                    let throw_away_at =
                        now_ms + u128::from(duration_ms) + u128::from(start_delay_ms);
                    self.playing_ff_effects.push((effect, throw_away_at));
                }
            }
        }
    }

    /// Update gamepad state.
    ///
    /// Should be called on each tick before reading gamepad state.
    pub fn poll(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        {
            for gamepad in self.gamepads.iter_mut() {
                gamepad.just_pressed_bits = 0;
            }

            while let Some(gilrs::Event { id, event, .. }) = self.gilrs_instance.next_event() {
                match event {
                    gilrs::EventType::Connected => {
                        if let Some(gamepad_idx) = self.find_or_insert(id) {
                            self.gamepads[gamepad_idx].connected = true;

                            for (zone, axis) in [
                                (0, gilrs::Axis::LeftStickX),
                                (1, gilrs::Axis::LeftStickY),
                                (2, gilrs::Axis::RightStickY),
                                (3, gilrs::Axis::RightStickY),
                            ] {
                                if let Some(code) = self.gilrs_instance.gamepad(id).axis_code(axis)
                                {
                                    self.deadzones[gamepad_idx][zone] = self
                                        .gilrs_instance
                                        .gamepad(id)
                                        .deadzone(code)
                                        .unwrap_or_default();
                                }
                            }
                        }
                    }
                    gilrs::EventType::Disconnected => {
                        if let Some(gamepad_idx) = self.find_or_insert(id) {
                            self.gamepads[gamepad_idx].connected = false;
                        }
                    }
                    gilrs::EventType::ButtonPressed(button, _code) => {
                        if let Some(gamepad_idx) = self.find_or_insert(id) {
                            if let Some(b) = Button::from_gilrs(button) {
                                let bit = 1 << (b as u32);
                                self.gamepads[gamepad_idx].pressed_bits |= bit;
                                self.gamepads[gamepad_idx].just_pressed_bits |= bit;
                            }
                        }
                    }
                    gilrs::EventType::ButtonReleased(button, _code) => {
                        if let Some(gamepad_idx) = self.find_or_insert(id) {
                            if let Some(b) = Button::from_gilrs(button) {
                                let bit = 1 << (b as u32);
                                self.gamepads[gamepad_idx].pressed_bits &= !bit;
                            }
                        }
                    }
                    gilrs::EventType::AxisChanged(axis, value, _code) => {
                        if let Some(gamepad_idx) = self.find_or_insert(id) {
                            if let Some(axis_idx) = match axis {
                                gilrs::Axis::LeftStickX => Some(0),
                                gilrs::Axis::LeftStickY => Some(1),
                                gilrs::Axis::RightStickX => Some(2),
                                gilrs::Axis::RightStickY => Some(3),
                                _ => None,
                            } {
                                let deadzone = self.deadzones[gamepad_idx][axis_idx];
                                self.gamepads[gamepad_idx].axes[axis_idx] =
                                    if value.abs() < deadzone {
                                        // Axis values within deadzone are 0:
                                        0.
                                    } else {
                                        // Adjust so that interval of magnitude is [0.0, 1.0]:
                                        value.signum().mul_add(-deadzone, value) / (1. - deadzone)
                                    };
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        #[cfg(target_family = "wasm")]
        {
            for gamepad in self.gamepads.iter_mut() {
                gamepad.last_pressed_bits = gamepad.pressed_bits;
            }
            #[cfg(not(feature = "wasm-bindgen"))]
            {
                let pointer = self.gamepads.as_ptr();
                unsafe { getGamepads(pointer) }
            }
            #[cfg(feature = "wasm-bindgen")]
            {
                #![allow(clippy::expect_used)]
                for gamepad in web_sys::window()
                    .expect("Unable to get window")
                    .navigator()
                    .get_gamepads()
                    .expect("Unable to get gamepads")
                    .iter()
                    .filter(|v| !v.is_null())
                {
                    let gamepad = web_sys::Gamepad::from(gamepad);
                    let mut pressed_bits: u32 = 0;
                    for (button_idx, button) in gamepad.buttons().iter().enumerate() {
                        let button = web_sys::GamepadButton::from(button);
                        if button.pressed() {
                            pressed_bits |= 1 << (button_idx as u32);
                        }
                    }
                    self.gamepads[gamepad.index() as usize].pressed_bits = pressed_bits;
                    self.gamepads[gamepad.index() as usize].connected = gamepad.connected();
                    for (axes_idx, axes_value) in gamepad
                        .axes()
                        .iter()
                        .map(|a| a.as_f64().expect("axes should be numbers"))
                        .enumerate()
                    {
                        self.gamepads[gamepad.index() as usize].axes[axes_idx] =
                            axes_value as f32 * if axes_idx % 2 == 1 { -1. } else { 1. };
                    }
                }
            }
        }
    }
}

/// A button on a gamepad.
///
/// Check for the current state of button presses on a gamepad using one of:
///
/// - [Gamepad::all_currently_pressed()]
/// - [Gamepad::all_just_pressed()]
/// - [Gamepad::is_currently_pressed()]
/// - [Gamepad::is_just_pressed()]
///
/// Different platforms call the buttons different things, see the below pictures for an overview, as
/// well as the individual button documentations for a comparison.
///
/// # Playstation
/// ![Playstation gamepad layout](https://www.gran-turismo.com/images/c/i17AZsIsc9rpTb.jpg)
///
/// # Switch
/// ![Switch gamepad layout](https://oyster.ignimgs.com/mediawiki/apis.ign.com/nintendo-nx/b/bb/Joycon.jpg?width=960)
///
/// # Xbox
/// ![Xbox gamepad layout](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2c/360_controller.svg/2880px-360_controller.svg.png)
///
/// # W3C Gamepad API standard gamepad layout:
/// ![Visual representation of a Standard Gamepad layout](https://w3c.github.io/gamepad/standard_gamepad.svg)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Button {
    /// Lowermost button in right cluster
    ///
    /// - Playstation: `X`/`Cross` button
    /// - Switch: `B` button
    /// - Xbox: `A` button
    /// - Gamepad API: `buttons[0]` / `Bottom button in right cluster`
    ActionDown,
    /// Rightmost button in right cluster
    ///
    /// - Gamepad API: `buttons[1]` / `Right button in right cluster`
    /// - Playstation: `O`/`Circle` button
    /// - Switch: `A`
    /// - Xbox: `B`
    ActionRight,
    /// Leftmost button in right cluster
    ///
    /// - Gamepad API: `buttons[2]` / `Left button in right cluster`
    /// - Playstation: `□`/`Square`
    /// - Switch: `Y`
    /// - Xbox: `X`
    ActionLeft,
    /// Topmost button in right cluster
    ///
    /// - Gamepad API: `buttons[3]` / `Top button in right cluster`
    /// - Playstation: `△`/`Triangle`
    /// - Switch: `X`
    /// - Xbox: `Y`
    ActionUp,
    /// Top left front button
    ///
    /// - Gamepad API: `buttons[4]` / `Top left front button`
    /// - Playstation: `L1`
    /// - Switch: `L`
    /// - Xbox: `LB` (`Left Bumper`)
    FrontLeftUpper,
    /// Top right front button
    ///
    /// - Gamepad API: `buttons[5]` / `Top right front button`
    /// - Playstation: `R1`
    /// - Switch: `R`
    /// - Xbox: `RB` (`Right Bumper`)
    FrontRightUpper,
    /// Bottom left front button
    ///
    /// - Gamepad API: `buttons[6]` / `Bottom left front button`
    /// - Playstation: `L2`
    /// - Switch: `ZL`
    /// - Xbox: `LT` (`Left Trigger`)
    FrontLeftLower,
    /// Bottom right front button
    ///
    /// - Gamepad API: `buttons[7]` / `Bottom right front button`
    /// - Playstation: `R2`
    /// - Switch: `ZR`
    /// - Xbox: `RT` (`Right Trigger`)
    FrontRightLower,
    /// Left button in center cluster - select/back
    ///
    /// - Gamepad API: `buttons[8]` / `Left button in center cluster`
    /// - Playstation: `SELECT`
    /// - Switch: `Capture`
    /// - Xbox: `RT` (`Right Trigger`)
    LeftCenterCluster,
    /// Right button in center cluster - start/forward.
    ///
    /// - Gamepad API: `buttons[9]` / `Right button in center cluster`
    /// - Playstation: `Start`
    /// - Switch: `Home`
    /// - Xbox: `Start`
    RightCenterCluster,
    /// Left stick pressed button.
    LeftStick,
    /// Right stick pressed button.
    RightStick,
    /// D-pad up button.
    DPadUp,
    /// D-pad down button.
    DPadDown,
    /// D-pad left button.
    DPadLeft,
    /// D-pad right button.
    DPadRight,
    /// Mode button.
    ///
    /// - Gamepad API: `buttons[16]` / `Center button in center cluster`
    Mode,
}

impl Button {
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::ActionDown,
            Self::ActionRight,
            Self::ActionLeft,
            Self::ActionUp,
            Self::FrontLeftUpper,
            Self::FrontRightUpper,
            Self::FrontLeftLower,
            Self::FrontRightLower,
            Self::LeftCenterCluster,
            Self::RightCenterCluster,
            Self::LeftStick,
            Self::RightStick,
            Self::DPadUp,
            Self::DPadDown,
            Self::DPadLeft,
            Self::DPadRight,
            Self::Mode,
        ]
        .into_iter()
    }

    #[cfg(not(target_family = "wasm"))]
    const fn from_gilrs(button: gilrs::Button) -> Option<Self> {
        Some(match button {
            gilrs::Button::South => Self::ActionDown,
            gilrs::Button::East => Self::ActionRight,
            gilrs::Button::West => Self::ActionLeft,
            gilrs::Button::North => Self::ActionUp,
            gilrs::Button::LeftTrigger => Self::FrontLeftUpper,
            gilrs::Button::RightTrigger => Self::FrontRightUpper,
            gilrs::Button::LeftTrigger2 => Self::FrontLeftLower,
            gilrs::Button::RightTrigger2 => Self::FrontRightLower,
            gilrs::Button::Select => Self::LeftCenterCluster,
            gilrs::Button::Start => Self::RightCenterCluster,
            gilrs::Button::LeftThumb => Self::LeftStick,
            gilrs::Button::RightThumb => Self::RightStick,
            gilrs::Button::DPadUp => Self::DPadUp,
            gilrs::Button::DPadDown => Self::DPadDown,
            gilrs::Button::DPadLeft => Self::DPadLeft,
            gilrs::Button::DPadRight => Self::DPadRight,
            gilrs::Button::Mode => Self::Mode,
            // Other:
            _ => {
                return None;
            }
        })
    }
}
