const WIDTH: usize = 401;
const HEIGHT: usize = 400;
const FG: macroquad::color::Color = macroquad::color::WHITE;
const BG: macroquad::color::Color = macroquad::color::BLANK;
const SECONDS_PER_TICK: f64 = 5.0;

use macroquad;

fn apply_rule(above: [bool; 3], rule: [bool; 8]) -> bool {
    let mut number = 0;
    number |= (above[2] as usize) << 2;
    number |= (above[1] as usize) << 1;
    number |= above[0] as usize;
    return rule[number];
}

fn generate_next_elementary_cellular_automata_row(
    row: [bool; WIDTH],
    border: bool,
    rule: [bool; 8],
) -> [bool; WIDTH] {
    let mut new_row = [false; WIDTH];
    let mut index = 1;
    for (a1, (a2, a3)) in row.iter().zip(row.iter().skip(1).zip(row.iter().skip(2))) {
        new_row[index] = apply_rule([*a1, *a2, *a3], rule);
        index += 1;
    }
    new_row[0] = apply_rule([border, row[0], row[1]], rule);
    new_row[WIDTH - 1] = apply_rule([row[WIDTH - 2], row[WIDTH - 1], border], rule);
    return new_row;
}

fn byte_to_ruleset(data: u8) -> [bool; 8] {
    let mut ruleset = [false; 8];
    for (i, block) in ruleset.iter_mut().enumerate() {
        let current_bit = (data & (1 << i)) != 0;
        *block = current_bit;
    }
    return ruleset;
}

fn generate_texture_with_rule(rule: u8) -> macroquad::texture::Texture2D {
    let mut testimage = macroquad::texture::Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BG);
    let testimage_data = testimage.get_image_data_mut();
    let current_rules = byte_to_ruleset(rule);
    //first row:
    let mut curent_row = [false; WIDTH];
    curent_row[WIDTH / 2] = true;
    for y in 0..HEIGHT {
        for (x, block) in curent_row.iter().enumerate() {
            if *block {
                testimage_data[x + y * WIDTH] = FG.into();
            }
        }
        curent_row =
            generate_next_elementary_cellular_automata_row(curent_row, false, current_rules);
    }
    let texture = macroquad::texture::Texture2D::from_image(&testimage);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);
    return texture;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut current_ruleset = 90;
    let mut texture = generate_texture_with_rule(current_ruleset);
    let mut last_tick = macroquad::time::get_time();
    loop {
        macroquad::window::clear_background(macroquad::color::BLACK);
        macroquad::shapes::draw_rectangle(
            macroquad::window::screen_width() / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            BG,
        );

        let delta = macroquad::time::get_time() - last_tick;
        let fraction_complete = delta / SECONDS_PER_TICK;
        let fraction_complete = fraction_complete as f32;
        macroquad::texture::draw_texture_ex(
            texture,
            -0.5 * macroquad::window::screen_width(),  //x
            -0.5 * macroquad::window::screen_height(), //y
            macroquad::color::DARKGREEN,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(
                    (
                        macroquad::window::screen_width() * 2.0,
                        macroquad::window::screen_height() * 2.0,
                    )
                        .into(),
                ),
                source: None,
                rotation: 2.0 * 3.14 * 0.5 * fraction_complete,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        macroquad::texture::draw_texture_ex(
            texture,
            0.0, //x
            0.0, //y
            macroquad::color::WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(
                    (
                        macroquad::window::screen_width(),
                        macroquad::window::screen_height(),
                    )
                        .into(),
                ),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        if delta >= SECONDS_PER_TICK {
            last_tick = macroquad::time::get_time();
            current_ruleset = current_ruleset.wrapping_add(1);
            texture = generate_texture_with_rule(current_ruleset);
        }

        macroquad::window::next_frame().await
    }
}
