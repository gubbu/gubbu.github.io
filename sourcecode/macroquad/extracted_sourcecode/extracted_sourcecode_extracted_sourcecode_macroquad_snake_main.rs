//the game is subdivided into tiles. The number of tiles is a whole number (WIDTH/HEIGHT):
const GAME_WIDTH: i32 = 10;
const GAME_HEIGHT: i32 = 10;
const GAME_WIDTH_FLOAT: f32 = GAME_WIDTH as f32;
const GAME_HEIGHT_FLOAT: f32 = GAME_HEIGHT as f32;
const GAME_MAP: &str = "##########
#S-------#
#--R-RRR-#
#--####--#
#---R#---#
#-RR-#-R-#
#----#---#
#-RR-#-R-#
#----#---#
##########";

struct SnakeStage {
    spawn_position: (i32, i32),
    reward_positions: Vec<(i32, i32)>,
    wall_map: [[bool; GAME_WIDTH as usize]; GAME_HEIGHT as usize],
    texture: macroquad::texture::Texture2D,
    current_reward_position: (i32, i32),
}

impl SnakeStage {
    fn select_random_reward_position(&mut self) {
        let random_number = macroquad::rand::rand() as usize;
        let randomly_selected = self.reward_positions[random_number % self.reward_positions.len()];
        self.current_reward_position = randomly_selected;
    }

    fn wall_collision(&self, position: (i32, i32)) -> bool {
        if position.0 < 0 || position.1 < 0 || position.0 >= GAME_WIDTH || position.1 >= GAME_HEIGHT
        {
            return false;
        } else {
            return self.wall_map[position.1 as usize][position.0 as usize];
        }
    }

    fn parse_game_map(game_map: &str) -> Self {
        let mut reward_positions = vec![];
        let mut spawn_position = (1, 1);
        let mut wall_map: [[bool; GAME_WIDTH as usize]; GAME_HEIGHT as usize] =
            [[false; GAME_WIDTH as usize]; GAME_HEIGHT as usize];
        let mut stage_image = macroquad::texture::Image::gen_image_color(
            GAME_WIDTH as u16,
            GAME_HEIGHT as u16,
            macroquad::color::BLUE,
        );
        for (y, line) in game_map.split("\n").enumerate() {
            for (x, block) in line.chars().enumerate() {
                if block == '#' {
                    stage_image.get_image_data_mut()[x + y * (GAME_WIDTH as usize)] =
                        [255u8, 0, 0, 255];
                    wall_map[y][x] = true;
                } else if block == 'R' {
                    reward_positions.push((x as i32, y as i32));
                } else if block == 'S' {
                    spawn_position = (x as i32, y as i32);
                }
            }
        }

        let stage_texture = macroquad::texture::Texture2D::from_image(&stage_image);
        stage_texture.set_filter(macroquad::texture::FilterMode::Nearest);

        let mut stage = SnakeStage {
            spawn_position: spawn_position,
            reward_positions: reward_positions,
            wall_map: wall_map,
            texture: stage_texture,
            current_reward_position: (0, 0),
        };
        stage.select_random_reward_position();
        stage
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut direction: (i32, i32) = (1, 0);
    //in seconds:
    let time_per_move: f64 = 0.4;
    let mut last_update: f64 = 0.0;
    let mut snake_stage = SnakeStage::parse_game_map(GAME_MAP);
    let mut snake: Vec<(i32, i32)> = vec![snake_stage.spawn_position];
    let mut is_game_over: bool = false;
    //macroquad::set_target_fps(30.0); //unimplemented!()
    //let mut context = &mut get_context().draw_context;
    loop {
        macroquad::texture::draw_texture_ex(
            snake_stage.texture,
            0.0,
            0.0,
            macroquad::color::WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(
                    macroquad::window::screen_width(),
                    macroquad::window::screen_height(),
                )),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        let block_width = macroquad::window::screen_width() / GAME_WIDTH_FLOAT;
        let block_height = macroquad::window::screen_height() / GAME_HEIGHT_FLOAT;
        //draw the snake:
        for (block_x, block_y) in snake.iter() {
            macroquad::shapes::draw_rectangle(
                *block_x as f32 * block_width,
                *block_y as f32 * block_height,
                block_width,
                block_height,
                macroquad::color::GREEN,
            );
        }
        if snake.len() >= 2 {
            for ((block_x, block_y), (next_x, next_y)) in snake.iter().zip(snake.iter().skip(1)) {
                let x1 = *block_x as f32 * block_width + block_width * 0.5;
                let y1 = *block_y as f32 * block_height + block_height * 0.5;
                let x2 = *next_x as f32 * block_width + block_width * 0.5;
                let y2 = *next_y as f32 * block_height + block_height * 0.5;
                macroquad::shapes::draw_line(
                    x1,
                    y1,
                    x2,
                    y2,
                    block_width.min(block_height) * 0.5,
                    macroquad::color::DARKGREEN,
                );
            }
        }

        //draw the reward: add a time based animation
        let radius_max = block_width.min(block_height) * 0.5;
        let radius_min = radius_max * 0.5;
        let reward_circle_radius = (macroquad::time::get_time() as f32).sin().abs()
            * (radius_max - radius_min)
            + radius_min;
        macroquad::shapes::draw_circle(
            snake_stage.current_reward_position.0 as f32 * block_width + block_width * 0.5,
            snake_stage.current_reward_position.1 as f32 * block_height + block_height * 0.5,
            reward_circle_radius,
            macroquad::color::YELLOW,
        );

        macroquad::text::draw_text(
            &format!("Score:\t{}", snake.len() - 1),
            20.0,
            20.0,
            40.0,
            macroquad::color::BLACK,
        );

        if is_game_over {
            macroquad::text::draw_text(
                "GAMEOVER! R To Restart",
                20.0,
                60.0,
                40.0,
                macroquad::color::BLACK,
            );
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::R) {
                direction = (1, 0);
                last_update = macroquad::time::get_time();
                snake = vec![snake_stage.spawn_position];
                is_game_over = false;
            }
        } else {
            //Controls
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::W) && direction != (0, 1)
            {
                direction = (0, -1);
            } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::S)
                && direction != (0, -1)
            {
                direction = (0, 1);
            } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::A)
                && direction != (1, 0)
            {
                direction = (-1, 0);
            } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::D)
                && direction != (-1, 0)
            {
                direction = (1, 0);
            }
            //fixed timesteps
            let elapsed = macroquad::time::get_time() - last_update;
            if elapsed > time_per_move {
                println!("update");
                last_update = macroquad::time::get_time();
                let (mut head_x, mut head_y) = snake[0];
                head_x += direction.0;
                head_y += direction.1;
                if snake.contains(&(head_x, head_y)) || snake_stage.wall_collision((head_x, head_y))
                {
                    // TODO: GAME OVER...
                    println!("GAMEOVER");
                    is_game_over = true;
                } else if (head_x, head_y) != snake_stage.current_reward_position {
                    snake.insert(0, (head_x, head_y));
                    snake.pop();
                } else {
                    println!("reward found");
                    snake.insert(0, (head_x, head_y));
                    snake_stage.select_random_reward_position();
                }
            }
        }

        macroquad::window::next_frame().await
    }
}
