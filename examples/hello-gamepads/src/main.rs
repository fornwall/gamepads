use gamepads::{Button, Gamepads};

fn main() {
    let mut gamepads = Gamepads::new();

    loop {
        gamepads.poll();

        for gamepad in gamepads.all() {
            // Use just_pressed_buttons() or currently_pressed_buttons().
            for button in gamepad.all_just_pressed() {
                println!("Button just pressed: {button:?}");
                match button {
                    Button::DPadUp => println!("Going up!"),
                    Button::ActionDown => println!("Shooting!"),
                    _ => {}
                }

                // Individual buttons can be checked using
                // is_just_pressed() / is_currently_pressed():
                if gamepad.is_currently_pressed(Button::FrontLeftLower) {
                    println!("Front left lower button is currently pressed");
                }
            }

            if gamepad.left_stick() != (0., 0.) {
                println!("Left stick: {:?}", gamepad.left_stick());
            }
            if gamepad.right_stick() != (0., 0.) {
                println!("Right stick: {:?}", gamepad.right_stick());
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
