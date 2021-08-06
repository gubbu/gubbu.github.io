extern crate macroquad;

/// generate an image from rgba color values represented by a f32 array.
fn texture_from_fn(
    width: u16,
    height: u16,
    image_fn: fn(u32, u32) -> [f32; 4],
) -> macroquad::texture::Texture2D {
    let mut image = macroquad::texture::Image::gen_image_color(
        width,
        height,
        macroquad::color::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        },
    );

    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let current_color = image_fn(x, y);
            let macroquad_color = macroquad::color::Color {
                r: current_color[0],
                g: current_color[1],
                b: current_color[2],
                a: current_color[3],
            };
            image.set_pixel(x, y, macroquad_color);
        }
    }

    return macroquad::texture::Texture2D::from_image(&image);
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let gradient_texture = texture_from_fn(164, 164, |x, _y| {
        let factor = x as f32 / 164.0;
        [factor; 4]
    });
    loop {
        macroquad::window::clear_background(macroquad::color::colors::BLACK);
        macroquad::texture::draw_texture(gradient_texture, 0.0, 0.0, macroquad::color::WHITE);
        macroquad::window::next_frame().await
    }
}
