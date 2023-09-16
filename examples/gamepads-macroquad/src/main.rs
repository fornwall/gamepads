struct Player {
    position: (f32, f32),
    gamepad_id: gamepads::GamepadId,
    color: macroquad::color::Color,
}

#[macroquad::main("MAIN")]
async fn main() {
    const VIRTUAL_WIDTH: f32 = 1.0;
    const VIRTUAL_HEIGHT: f32 = 1.0;

    let mut players: Vec<Player> = Vec::new();

    let mut pads = gamepads::Gamepads::new();
    let mut fullscreen = false;

    loop {
        pads.poll();

        let mut clear_color = macroquad::color::BLACK;

        for gamepad in pads.all() {
            #[allow(clippy::option_if_let_else)]
            let player =
                if let Some(player) = players.iter_mut().find(|p| p.gamepad_id == gamepad.id()) {
                    player
                } else {
                    let new_player = Player {
                        position: (VIRTUAL_WIDTH / 2., VIRTUAL_HEIGHT / 2.),
                        gamepad_id: gamepad.id(),
                        color: macroquad::color::Color::from_hex(DISTINCT_COLORS[0]),
                    };
                    players.push(new_player);
                    players.last_mut().unwrap()
                };

            player.position.0 =
                (player.position.0 + 0.01 * gamepad.left_stick_x()).rem_euclid(VIRTUAL_WIDTH);
            player.position.1 =
                (player.position.1 + 0.01 * gamepad.left_stick_y()).rem_euclid(VIRTUAL_HEIGHT);
            player.position.0 =
                (player.position.0 + 0.04 * gamepad.right_stick_x()).rem_euclid(VIRTUAL_WIDTH);
            player.position.1 =
                (player.position.1 + 0.04 * gamepad.right_stick_y()).rem_euclid(VIRTUAL_HEIGHT);

            for button_type in gamepad.all_just_pressed() {
                player.color =
                    macroquad::color::Color::from_hex(DISTINCT_COLORS[button_type as usize]);
            }

            if gamepad.is_just_pressed(gamepads::Button::Mode) {
                fullscreen = !fullscreen;
                macroquad::window::set_fullscreen(fullscreen);
            }

            if gamepad.is_currently_pressed(gamepads::Button::FrontRightLower) {
                clear_color = macroquad::color::VIOLET;
            }

            if gamepad.is_just_pressed(gamepads::Button::ActionDown) {
                pads.rumble(gamepad.id(), 1000, 0, 0.8, 0.8);
            }
            if gamepad.is_just_pressed(gamepads::Button::ActionRight) {
                pads.rumble(gamepad.id(), 1000, 0, 0.1, 0.1);
            }
        }

        macroquad::prelude::clear_background(clear_color);

        let camera = macroquad::camera::Camera2D::from_display_rect(macroquad::math::Rect::new(
            0.,
            0.,
            VIRTUAL_WIDTH,
            VIRTUAL_HEIGHT,
        ));
        macroquad::camera::set_camera(&camera);

        for player in players.iter() {
            let size = 0.05;
            macroquad::shapes::draw_rectangle(
                player.position.0 - size / 2.,
                player.position.1 - size / 2.,
                size,
                size,
                player.color,
            );
        }

        macroquad::window::next_frame().await;
    }
}

const DISTINCT_COLORS: [u32; 20] = [
    0xFFF6768E, //Strong Purplish Pink
    0xFF00538A, //Strong Blue
    0xFFF4C800, //Vivid Greenish Yellow
    0xFF7F180D, //Strong Reddish Brown
    0xFFFF6800, //Vivid Orange
    0xFFA6BDD7, //Very Light Blue
    0xFFC10020, //Vivid Red
    0xFFCEA262, //Grayish Yellow
    0xFFFFB300, //Vivid Yellow
    0xFF803E75, //Strong Purple
    0xFF817066, //Medium Gray
    0xFF007D34, //Vivid Green
    0xFFFF7A5C, //Strong Yellowish Pink
    0xFF53377A, //Strong Violet
    0xFFFF8E00, //Vivid Orange Yellow
    0xFFB32851, //Strong Purplish Red
    0xFF93AA00, //Vivid Yellowish Green
    0xFF593315, //Deep Yellowish Brown
    0xFFF13A13, //Vivid Reddish Orange
    0xFF232C16, //Dark Olive Green
];
