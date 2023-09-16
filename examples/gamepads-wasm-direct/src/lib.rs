extern "C" {
    // Host javascript function.
    fn log_gamepad_state(
        gamepad_id: u8,
        buttons_bitmask: u32,
        left_x: f32,
        left_y: f32,
        right_x: f32,
        right_y: f32,
    );
}

#[no_mangle]
pub extern "C" fn check_gamepads() {
    let mut gamepads = gamepads::Gamepads::new();

    gamepads.poll();
    for gamepad in gamepads.all() {
        let mut buttons_bitmask = 0;
        for button in gamepad.all_currently_pressed() {
            buttons_bitmask |= 1 << (button as u32);
        }

        unsafe {
            log_gamepad_state(
                gamepad.id().value(),
                buttons_bitmask,
                gamepad.left_stick_x(),
                gamepad.left_stick_y(),
                gamepad.right_stick_x(),
                gamepad.right_stick_y(),
            );
        }
    }
}
