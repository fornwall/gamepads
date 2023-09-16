[![CI](https://github.com/fornwall/gamepads/actions/workflows/ci.yml/badge.svg)](https://github.com/fornwall/gamepads/actions/workflows/ci.yml)
[![Docs](https://docs.rs/gamepads/badge.svg)](https://docs.rs/gamepads/)
[![Crates.io version](https://img.shields.io/crates/v/gamepads.svg)](https://crates.io/crates/gamepads)

# gamepads
Rust gamepad input library with a focus on ease of use.

```rust
use gamepads::Gamepads;

fn main() {
    let mut gamepads = Gamepads::new();

    loop {
        gamepads.poll();

        for gamepad in gamepads.all() {
            println!("Gamepad id: {:?}", gamepad.id());
            for button in gamepad.all_currently_pressed() {
                println!("Pressed button: {:?}", button);
            }
            println!("Left thumbstick: {:?}", gamepad.left_stick());
            println!("Right thumbstick: {:?}", gamepad.right_stick());
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
   }
}
```

See the [crate documentation](https://docs.rs/gamepads/latest/gamepads/) and the [examples](https://github.com/fornwall/gamepads/tree/main/examples/) for documentation and sample code.

## What it is

- On desktop this library is implemented on top of [gilrs](https://crates.io/crates/gilrs).
- On web this is implemented on top of the [Gamepad API](https://www.w3.org/TR/gamepad/) exposed by browsers, including support for haptic feedback (aka "dual rumble" or "force feedback").
  - It can be used in a `wasm-bindgen`-using project without any setup necessary.
  - It can be used without `wasm-bindgen` (by specifying `default-features = false`), allowing it to be used as a `macroquad` plugin (see more below) or in a direct wasm build ([example](https://github.com/fornwall/gamepads/tree/main/examples/gamepads-wasm-direct)).

## How to use as a macroquad plugin
For non-web targets, nothing special needs to be done to use this library with [macroquad](https://github.com/not-fl3/macroquad). But for a web build to work properly, two things needs to be done.

First, since `macroquad` does not use `wasm-bindgen`, that feature in `gamepads` needs to be turned off by setting `default-features = false`:

```toml
gamepads = { version = "*", default-features = false }
```

Second, a javascript plug-in ([source](https://github.com/fornwall/gamepads/blob/main/js/gamepads-src-0.1.js)) needs to be registered in the page embedding the built wasm file:

```html
<script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
<script src="https://fornwall.github.io/gamepads/js/macroquad-gamepads-0.1.js"></script>
<script>
load("your-wasm-file.wasm");
</script>
```

See the [gamepads-macroquad](https://github.com/fornwall/gamepads/tree/main/examples/gamepads-macroquad) example.

# Feedback
Please [report any issues found](https://github.com/fornwall/gamepads/issues) or [discuss questions and ideas](https://github.com/fornwall/gamepads/discussions)!
