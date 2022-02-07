extern crate macroquad;

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn light_poliferation(blocks: [[bool; WIDTH]; HEIGHT], light_levels: &mut [[u8; WIDTH]; HEIGHT]) {
    //blocks outside of the loaded world have a light level of 15:
    let outside = 0;
    for x in 0..WIDTH as i32 {
        for y in 0..HEIGHT as i32 {
            let xu = x as usize;
            let yu = y as usize;
            //light already polifarated there, skip it.
            if light_levels[yu][xu] == 15 {
                continue;
            }
            //dont calculate light level under block:
            if blocks[yu][xu] {
                continue;
            }
            //von neumann neighbors in 2D: 4 neighbors
            let mut neuman_neighbor_maximum_light_level = 0;
            //iterate over von neumann neighborhood of current cell
            for neighbor in [[-1, 0], [1, 0], [0, 1], [0, -1]].iter() {
                let evaluate_at_x = x + neighbor[0];
                let evaluate_at_y = y + neighbor[1];
                let mut light_level = outside;
                //bounds check
                if evaluate_at_x >= 0
                    && evaluate_at_y >= 0
                    && evaluate_at_x < WIDTH as i32
                    && evaluate_at_y < HEIGHT as i32
                {
                    let evaluate_at_x = evaluate_at_x as usize;
                    let evaluate_at_y = evaluate_at_y as usize;
                    light_level = light_levels[evaluate_at_y][evaluate_at_x];
                }
                neuman_neighbor_maximum_light_level =
                    neuman_neighbor_maximum_light_level.max(light_level);
            }

            if neuman_neighbor_maximum_light_level > 0 {
                let current_light_level = light_levels[yu][xu];
                let new_light_lvl = neuman_neighbor_maximum_light_level - 1;
                light_levels[yu][xu] = current_light_level.max(new_light_lvl);
            }
        }
    }
}

fn update_lightlevels(
    blocks: [[bool; WIDTH]; HEIGHT],
    light_levels: &mut [[u8; WIDTH]; HEIGHT],
    torches: &Vec<[usize; 2]>,
) {
    // 0 fill
    *light_levels = [[0; WIDTH]; HEIGHT];
    //1. fill in the sky light.
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if blocks[y][x] {
                break;
            }
            light_levels[y][x] = 15;
        }
    }

    //place the torches:
    for [torchx, torchy] in torches {
        if light_levels[*torchy][*torchx] == 15 {
            continue;
        }
        light_levels[*torchy][*torchx] = 14;
    }

    //apply light polifaration ... 15 times
    for _i in 0..15 {
        light_poliferation(blocks, light_levels);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut box_list = [[false; WIDTH]; HEIGHT];
    let mut light_levels = [[0u8; WIDTH]; HEIGHT];
    let mut torches: Vec<[usize; 2]> = vec![];

    update_lightlevels(box_list, &mut light_levels, &torches);

    loop {
        let box_width = macroquad::window::screen_width() / WIDTH as f32;
        let box_height = macroquad::window::screen_height() / HEIGHT as f32;
        macroquad::window::clear_background(macroquad::color::colors::WHITE);
        for (y, row) in box_list.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let x_pos = x as f32 * box_width;
                let y_pos = y as f32 * box_height;

                if *block {
                    macroquad::shapes::draw_rectangle(
                        x_pos,
                        y_pos,
                        box_width,
                        box_height,
                        macroquad::color::RED,
                    );
                    continue;
                }

                let light_level = light_levels[y][x];
                if light_level != 15 {
                    let shaddow_alpha = 1.0 - light_level as f32 / 15.0;
                    macroquad::shapes::draw_rectangle(
                        x_pos,
                        y_pos,
                        box_width,
                        box_height,
                        macroquad::color::Color::new(0.0, 0.0, 0.0, shaddow_alpha),
                    );
                }

                let font_size = box_height / 1.5;

                macroquad::text::draw_text(
                    &format!("{}", light_levels[y][x]),
                    x_pos,
                    y_pos + font_size,
                    font_size,
                    macroquad::color::GREEN,
                );
            }
        }

        for [torchx, torchy] in torches.iter() {
            let torchx = *torchx as f32 * box_width + box_width * 0.5;
            let torchy = *torchy as f32 * box_height + box_height * 0.5;
            let radius = box_width.min(box_height) * 0.25;
            macroquad::shapes::draw_poly(torchx, torchy, 3, radius, 0.0, macroquad::color::RED)
        }

        let (mousex, mousey) = macroquad::input::mouse_position();

        let box_x = (mousex / box_width).floor() * box_width;
        let box_y = (mousey / box_height).floor() * box_height;

        macroquad::shapes::draw_rectangle(
            box_x,
            box_y,
            box_width,
            box_height,
            macroquad::color::Color::new(0.0, 1.0, 0.0, 0.5),
        );

        if macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left) {
            let x_index = (mousex / box_width).floor() as usize;
            let y_index = (mousey / box_height).floor() as usize;

            if torches.contains(&[x_index, y_index]) {
                torches.retain(|x| *x != [x_index, y_index]);
            } else {
                if box_list[y_index][x_index] {
                    torches.push([x_index, y_index]);
                }
                box_list[y_index][x_index] = !box_list[y_index][x_index];
            }

            update_lightlevels(box_list, &mut light_levels, &torches);
        }

        macroquad::window::next_frame().await
    }
}
