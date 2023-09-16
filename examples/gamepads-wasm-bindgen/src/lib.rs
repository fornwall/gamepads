use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn check_gamepads() {
    let window = web_sys::window().expect("no global `window` exists");

    let mut gamepads = gamepads::Gamepads::new();

    let closure = Closure::new(move || {
        gamepads.poll();
        for gamepad in gamepads.all() {
            for button in gamepad.all_currently_pressed() {
                let text = format!("Button: {:?}", button);
                web_sys::console::log_1(&text.into());
            }
            let text = format!("left: {:?}", gamepad.left_stick());
            web_sys::console::log_1(&text.into());
            let text = format!("right: {:?}", gamepad.right_stick());
            web_sys::console::log_1(&text.into());
            if gamepad.is_currently_pressed(gamepads::Button::DPadRight) {
                gamepads.rumble(gamepad.id(), 500, 0, 0.5, 0.4);
            }
        }
    });
    web_sys::Window::set_interval_with_callback_and_timeout_and_arguments_0(
        &window,
        closure.as_ref().unchecked_ref(),
        1000,
    )
    .unwrap();
    closure.forget();
}
