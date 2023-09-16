#[cfg(target_family = "wasm")]
#[cfg(feature = "wasm-bindgen")]
pub fn play_effect(
    gamepad_id: u8,
    duration_ms: u32,
    start_delay_ms: u32,
    strong_magnitude: f32,
    weak_magnitude: f32,
) {
    #![allow(clippy::expect_used)]
    use wasm_bindgen::JsValue;

    for gamepad in web_sys::window()
        .expect("Unable to get window")
        .navigator()
        .get_gamepads()
        .expect("Unable to get gamepads")
        .iter()
        .filter(|v| !v.is_null())
    {
        let typed_gamepad = web_sys::Gamepad::from(gamepad);
        if typed_gamepad.index() == u32::from(gamepad_id) {
            if let Ok(vibration_actuator) =
                js_sys::Reflect::get(&typed_gamepad, &JsValue::from_str("vibrationActuator"))
            {
                if let Ok(play_effect) =
                    js_sys::Reflect::get(&vibration_actuator, &JsValue::from_str("playEffect"))
                {
                    use wasm_bindgen::JsCast;
                    let arguments_list = js_sys::Array::new();
                    arguments_list.push(&JsValue::from_str("dual-rumble"));
                    let arg_obj = js_sys::Object::new();
                    let _ = js_sys::Reflect::set(&arg_obj, &"duration".into(), &duration_ms.into());
                    let _ = js_sys::Reflect::set(
                        &arg_obj,
                        &"startDelay".into(),
                        &start_delay_ms.into(),
                    );
                    let _ = js_sys::Reflect::set(
                        &arg_obj,
                        &"strongMagnitude".into(),
                        &strong_magnitude.into(),
                    );
                    let _ = js_sys::Reflect::set(
                        &arg_obj,
                        &"weakMagnitude".into(),
                        &weak_magnitude.into(),
                    );
                    arguments_list.push(&arg_obj);

                    let _ = js_sys::Reflect::apply(
                        play_effect.unchecked_ref(),
                        &vibration_actuator,
                        &arguments_list,
                    );
                }
            }
        }
    }
}
