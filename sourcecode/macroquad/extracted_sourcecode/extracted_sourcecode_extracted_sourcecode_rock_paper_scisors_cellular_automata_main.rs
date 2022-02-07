use macroquad::prelude::*;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;
const EMPTY: u8 = 0;
const THRESHHOLD: usize = 3;
const GAMEWIDTH: usize = 125;
const GAMEHEIGHT: usize = 125;
type GAME = [[u8; GAMEWIDTH]; GAMEHEIGHT];

fn winner(player1: u8, player2: u8) -> u8 {
    if player1 == PAPER && player2 == ROCK {
        return player1;
    } else if player1 == SCISSORS && player2 == PAPER {
        return player1;
    } else if player1 == ROCK && player2 == SCISSORS {
        return player1;
    }
    player2
}

fn randomize_with_seed(seed: u64) -> GAME {
    macroquad::rand::srand(seed);
    let mut game_state = [[0; GAMEWIDTH]; GAMEHEIGHT];
    //populate randomly
    //macroquad::rand::srand(10);
    for y in 0..GAMEHEIGHT {
        for x in 0..GAMEWIDTH {
            game_state[y][x] = macroquad::rand::gen_range(1, 4);
        }
    }
    return game_state;
}

fn simulation(gamestate: GAME) -> GAME {
    let mut new_gamestate: GAME = [[0; GAMEWIDTH]; GAMEHEIGHT];
    for y in 1..GAMEHEIGHT - 1 {
        for x in 1..GAMEWIDTH - 1 {
            let player1 = gamestate[y][x];
            let mut loss_count = 0;
            let mut loosing_against = 0;
            for yi in -1..2 {
                for xi in -1..2 {
                    if xi == yi && xi == 0 {
                        continue;
                    }
                    let x_index = x as i32 + xi;
                    let y_index = y as i32 + yi;
                    let x_index = x_index as usize;
                    let y_index = y_index as usize;
                    let player2 = gamestate[y_index][x_index];
                    //if the neighboring cell is empty: self replication
                    if player2 == EMPTY {
                        new_gamestate[y_index][x_index] = player1;
                        continue;
                    }
                    if player1 == player2 {
                        continue;
                    }
                    if player2 == winner(player1, player2) {
                        loss_count += 1;
                        loosing_against = player2;
                    }
                }
            }
            if loss_count >= THRESHHOLD {
                new_gamestate[y][x] = loosing_against;
            } else {
                new_gamestate[y][x] = player1;
            }
        }
    }
    return new_gamestate;
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game_state = randomize_with_seed(10);
    let max_digits = 4;
    //populate randomly
    //macroquad::rand::srand(10);
    let mut digits: Vec<usize> = vec![];
    let mut last_tick = macroquad::time::get_time();
    loop {
        let delta_time = macroquad::time::get_time() - last_tick;
        if delta_time < 0.25 {
            continue;
        }

        let width = macroquad::window::screen_width();
        let height = macroquad::window::screen_height();
        let block_width = width / (GAMEWIDTH - 2) as f32;
        let block_height = height / (GAMEHEIGHT - 2) as f32;
        macroquad::window::clear_background( macroquad::color::Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0});
        for y in 1..GAMEHEIGHT - 1 {
            for x in 1..GAMEWIDTH - 1 {
                let x_coor = (x - 1) as f32 * block_width;
                let y_coor = (y - 1) as f32 * block_height;
                let tile = game_state[y][x];
                if tile == SCISSORS {
                    macroquad::shapes::draw_rectangle(
                        x_coor,
                        y_coor,
                        block_width,
                        block_height,
                        macroquad::color::Color{r: 1.0, g: 0.0, b: 0.0, a: 1.0}
                    );
                } else if tile == ROCK {
                    macroquad::shapes::draw_rectangle(
                        x_coor,
                        y_coor,
                        block_width,
                        block_height,
                        macroquad::color::Color{r: 0.0, g: 1.0, b: 0.0, a: 1.0}
                    );
                }
            }
        }

        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key1) {
            digits.push(1);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key2) {
            digits.push(2);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key3) {
            digits.push(3);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key4) {
            digits.push(4);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key5) {
            digits.push(5);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key6) {
            digits.push(6);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key7) {
            digits.push(7);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key8) {
            digits.push(8);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key9) {
            digits.push(9);
        } else if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Key0) {
            digits.push(0);
        }
        if digits.len() == max_digits {
            let mut num = 0;
            //println!("{:?}", digits);
            for (i, digit) in digits.iter().rev().enumerate() {
                //println!("{}. digit: {}", i, digit);
                num += digit * 10usize.pow(i as u32);
            }
            println!(" change speed : {}", num);
            digits.clear();
            game_state = randomize_with_seed(num as u64);
        }
        game_state = simulation(game_state);
        last_tick = macroquad::time::get_time();
        macroquad::window::next_frame().await
    }
}
