#[macroquad::main("MAIN")]
async fn main() {
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

    const VIRTUAL_WIDTH: f32 = 1.0;
    const VIRTUAL_HEIGHT: f32 = 1.0;

    let mut color_idx = 0;

    let mut position = (VIRTUAL_WIDTH / 2., VIRTUAL_HEIGHT / 2.);

    let mut pads = gamepads::Gamepads::new();

    loop {
        macroquad::prelude::clear_background(macroquad::color::BLACK);

        let camera = macroquad::camera::Camera2D::from_display_rect(macroquad::math::Rect::new(
            0.,
            0.,
            VIRTUAL_WIDTH,
            VIRTUAL_HEIGHT,
        ));
        macroquad::camera::set_camera(&camera);

        for gamepad in pads.list() {
            let axes = gamepad.axes();
            position.0 = (position.0 + 0.01 * axes[0]).rem_euclid(VIRTUAL_WIDTH);
            position.1 = (position.1 - 0.01 * axes[1]).rem_euclid(VIRTUAL_HEIGHT);
            position.0 = (position.0 + 0.01 * axes[2]).rem_euclid(VIRTUAL_WIDTH);
            position.1 = (position.1 - 0.01 * axes[3]).rem_euclid(VIRTUAL_HEIGHT);
            for (i, pressed) in gamepad.buttons().enumerate() {
                if pressed {
                    color_idx = i;
                }
            }
        }

        let size = 0.05;
        macroquad::shapes::draw_rectangle(
            position.0 - size / 2.,
            position.1 - size / 2.,
            size,
            size,
            macroquad::color::Color::from_hex(DISTINCT_COLORS[color_idx]),
        );
        macroquad::window::next_frame().await;
    }
}
