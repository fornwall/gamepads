[![CI](https://github.com/fornwall/gamepads/actions/workflows/ci.yml/badge.svg)](https://github.com/fornwall/gamepads/actions/workflows/ci.yml)
[![Docs](https://docs.rs/gamepads/badge.svg)](https://docs.rs/gamepads/)
[![Crates.io version](https://img.shields.io/crates/v/gamepads.svg)](https://crates.io/crates/gamepads)

# gamepads
A crate to expose gamepad information in browsers using the [Gamepad API](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API/Using_the_Gamepad_API) exposed by browsers.


Add the dependency as:

```toml
[dependencies]
gamepads = "0.1.0"
```

On web, this crate uses a small javascript function, making it possible to use as a [macroquad](https://github.com/not-fl3/macroquad) plugin - more about that below.

## Macroquad plugin
A [macroquad](https://github.com/not-fl3/macroquad) plugin to access gamepad information in browsers.

First add the javascript, either bundling [macroquad-gamepads.js](https://fornwall.github.io/gamepads/macroquad-gamepads.js) or embedding it after:

```html
<script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
<script src="https://fornwall.github.io/gamepads/macroquad-gamepads.js"></script>
<script>
  load("your-wasm-file.wasm");
</script>
```

# Report issues
Please [report any issues found](https://github.com/fornwall/gamepads/issues)!
