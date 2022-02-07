extern crate macroquad; // 0.3.5
extern crate qrcode; // 0.12.0

#[macroquad::main("text2qrcode")]
async fn main() {
    let mut current_string = String::from("Enter your text in this Editbox!");
    let quiet_zone = 64.0;
    let mut qr_code_width = 0;
    let mut qr_code_string = None;
    use macroquad::ui::hash;
    loop {
        macroquad::window::clear_background(macroquad::color::colors::WHITE);
        if macroquad::ui::root_ui().editbox(
            hash!(),
            macroquad::math::Vec2::new(macroquad::window::screen_width(), 30.),
            &mut current_string,
        ) {
            //println!("Editbodx");
            let code = qrcode::QrCode::new(&current_string).unwrap();
            let string = code
                .render()
                .light_color(' ')
                .dark_color('#')
                .quiet_zone(false)
                .build();
            qr_code_string = Some(string);
            qr_code_width = code.width();
        }

        if let Some(qr_string) = &qr_code_string {
            let qr_code_height = qr_string.len() / (qr_code_width + 1);
            let block_width =
                (macroquad::window::screen_width() - 2.0 * quiet_zone) / qr_code_width as f32;
            let block_height =
                (macroquad::window::screen_height() - 2.0 * quiet_zone) / qr_code_height as f32;
            let block_size = block_width.min(block_height);
            let mut current_x = 0;
            let mut current_y = 0;
            for block in qr_string.chars() {
                if block == '\n' {
                    current_x = 0;
                    current_y += 1;
                    continue;
                } else if block == '#' {
                    let block_x = current_x as f32 * block_size + quiet_zone;
                    let block_y = current_y as f32 * block_size + quiet_zone;
                    macroquad::shapes::draw_rectangle(
                        block_x,
                        block_y,
                        block_size,
                        block_size,
                        macroquad::color::colors::BLACK,
                    );
                }
                current_x += 1;
            }
        }

        macroquad::window::next_frame().await
    }
}
