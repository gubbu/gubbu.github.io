extern crate macroquad;

const FOREGROUND: macroquad::color::Color = macroquad::color::LIGHTGRAY;
const TEXTCOLOR: macroquad::color::Color = macroquad::color::RED;
const BACKGROUND: macroquad::color::Color = macroquad::color::DARKBLUE;
const TEXTFADEOUT_SECONDS: f64 = 1.5;
const TEXTSIZE: f32 = 30.0;

mod tictactoe {
    // X = -1
    // O = +1
    // 0 = Neutral
    type Field = [[i8; 3]; 3];

    // returns 0, if the current state is a draw
    pub fn winner(field: &Field) -> i8 {
        //check vertical lines:
        for x in 0..3 {
            let mut sum = 0;
            for y in 0..3 {
                sum += field[y][x];
            }
            if sum.abs() == 3 {
                return sum / 3;
            }
        }
        //check horizontal lines:
        for y in 0..3 {
            let mut sum = 0;
            for x in 0..3 {
                sum += field[y][x];
            }
            if sum.abs() == 3 {
                return sum / 3;
            }
        }
        //check diagonal 1:
        let mut sum = 0;
        for i in 0..3 {
            sum += field[i][i];
        }
        if sum.abs() == 3 {
            return sum / 3;
        }
        //check diagonal 2:
        let mut sum = 0;
        for i in 0..3 {
            sum += field[2 - i][i];
        }
        if sum.abs() == 3 {
            return sum / 3;
        }
        return 0;
    }

    pub fn is_draw(field: &Field) -> bool {
        if winner(field) == 0 {
            for row in field.iter() {
                for tile in row.iter() {
                    if *tile == 0 {
                        return false;
                    }
                }
            }
            return true;
        }
        return false;
    }

    fn is_winning_move(player: i8, field: &Field) -> bool {
        player == winner(field)
    }

    pub enum AIResult {
        AIWins,
        AIEnemyWins,
        EnemyMove,
        Draw,
    }

    //player arg: which id represents the AI?
    pub fn simple_ai_decision(player: i8, mut field: Field) -> (AIResult, Field) {
        //check for a winning move for self
        for (y, row) in field.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    let mut new_field = field.clone();
                    new_field[y][x] = player;
                    if is_winning_move(player, &new_field) {
                        return (AIResult::AIWins, new_field);
                    }
                }
            }
        }
        //check for a winning move for the enemy, block it, if possible
        for (y, row) in field.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    let mut new_field = field.clone();
                    new_field[y][x] = -player;
                    if is_winning_move(-player, &new_field) {
                        new_field[y][x] = player;
                        return (AIResult::EnemyMove, new_field);
                    }
                }
            }
        }
        // check if its a draw
        if is_draw(&field) {
            return (AIResult::Draw, field);
        }
        //else: make a random move
        let mut possible_moves = vec![];
        for (y, row) in field.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    possible_moves.push((x, y));
                }
            }
        }
        if possible_moves.len() == 0 {
            let mut result = AIResult::Draw;
            let winner = winner(&field);
            if winner == player {
                result = AIResult::AIWins;
            } else if winner == -player {
                result = AIResult::AIEnemyWins;
            }
            return (result, field);
        }
        let random_move: (usize, usize) =
            possible_moves[macroquad::rand::rand() as usize % possible_moves.len()];
        field[random_move.1][random_move.0] = player;
        if is_draw(&field) {
            return (AIResult::Draw, field);
        } else {
            return (AIResult::EnemyMove, field);
        }
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut field = [[0i8; 3]; 3];
    let mut announcer_message: Option<&str> = None;
    let mut ai_score: u32 = 0;
    let mut human_score: u32 = 0;
    let mut draw_count: u32 = 0;
    let mut text_fadeout_time: f64 = 0.0;
    loop {
        macroquad::window::clear_background(BACKGROUND);
        let block_width = macroquad::window::screen_width() / 3.0;
        let block_height = macroquad::window::screen_height() / 3.0;

        //draw vertical grid lines:
        for i in 0..4 {
            let x_pos = i as f32 * block_width;
            macroquad::shapes::draw_line(
                x_pos,
                0.0,
                x_pos,
                macroquad::window::screen_height(),
                5.0,
                FOREGROUND,
            );
        }
        //draw horizontal grid lines
        for i in 0..4 {
            let y_pos = i as f32 * block_height;
            macroquad::shapes::draw_line(
                0.0,
                y_pos,
                macroquad::window::screen_width(),
                y_pos,
                5.0,
                FOREGROUND,
            );
        }

        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            if announcer_message.is_none() {
                let (x, y) = macroquad::input::mouse_position();
                let x_grid = (x.abs() / block_width.abs()) as usize;
                let y_grid = (y.abs() / block_height.abs()) as usize;
                if x_grid < 3 && y_grid < 3 {
                    if field[y_grid][x_grid] == 0 {
                        //human player makes his move.
                        field[y_grid][x_grid] = 1;
                        //ai makes a move:
                        //before making the AI decision: check if the current move leads to a win for 1:
                        if tictactoe::winner(&field) != 1 {
                            let (winner, new_field) = tictactoe::simple_ai_decision(-1, field);
                            field = new_field;
                            match winner {
                                tictactoe::AIResult::AIWins => {
                                    announcer_message = Some("Bot Won! :/");
                                    println!("{:?}", announcer_message);
                                    text_fadeout_time =
                                        macroquad::time::get_time() + TEXTFADEOUT_SECONDS;
                                    ai_score += 1;
                                }
                                tictactoe::AIResult::AIEnemyWins => {
                                    //makes no sence here because the ai can only make a move if the player did not win
                                    announcer_message = Some("You Won! :)");
                                    println!("{:?}", announcer_message);
                                    text_fadeout_time =
                                        macroquad::time::get_time() + TEXTFADEOUT_SECONDS;
                                    human_score += 1;
                                }
                                tictactoe::AIResult::Draw => {
                                    announcer_message = Some("It Is a Draw! -_-");
                                    println!("{:?}", announcer_message);
                                    text_fadeout_time =
                                        macroquad::time::get_time() + TEXTFADEOUT_SECONDS;
                                    draw_count += 1;
                                }
                                _ => {}
                            }
                        } else {
                            announcer_message = Some("You Won! :)");
                            println!("{:?}", announcer_message);
                            text_fadeout_time = macroquad::time::get_time() + TEXTFADEOUT_SECONDS;
                            human_score += 1;
                        }
                    }
                }
            }
            //println!("mousebutton over {}, {}", x_grid, y_grid);
        }
        //calculate alpha of the text:
        let mut alpha = 1.0;
        if announcer_message.is_some() {
            let elapsed_time = text_fadeout_time - macroquad::time::get_time();
            let elapsed_percentage = elapsed_time.max(0.0) / TEXTFADEOUT_SECONDS;
            alpha = elapsed_percentage as f32;
        }
        let element_color = macroquad::color::Color {
            r: FOREGROUND.r,
            g: FOREGROUND.g,
            b: FOREGROUND.b,
            a: alpha,
        };
        //draw the field:
        for (y, row) in field.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let center_x = x as f32 * block_width + block_width * 0.5;
                let center_y = y as f32 * block_height + block_height * 0.5;
                //the human player/You are a circle
                if *block == 1 {
                    macroquad::shapes::draw_circle(
                        center_x,
                        center_y,
                        block_width.min(block_height) * 0.4,
                        element_color,
                    )
                } else if *block == -1 {
                    let half_width = block_width * 0.4;
                    let half_height = block_height * 0.4;
                    //draw a cross ...
                    //one diagonal:
                    macroquad::shapes::draw_line(
                        center_x - half_width,
                        center_y - half_height,
                        center_x + half_width,
                        center_y + half_height,
                        2.5,
                        element_color,
                    );
                    //other diagonal
                    macroquad::shapes::draw_line(
                        center_x - half_width,
                        center_y + half_height,
                        center_x + half_width,
                        center_y - half_height,
                        2.5,
                        element_color,
                    );
                }
            }
        }

        //draw announcement in the screen center:
        if let Some(message) = announcer_message {
            let textdimensions = macroquad::text::measure_text(message, None, 1, TEXTSIZE);
            let text_width = textdimensions.width;
            let text_height = textdimensions.height;
            let (screen_center_x, screen_center_y) = (
                macroquad::window::screen_width() * 0.5,
                macroquad::window::screen_height() * 0.5,
            );
            let x_pos = screen_center_x - text_width * 0.5;
            let y_pos = screen_center_y - text_height * 0.5;
            let text_color = macroquad::color::Color {
                r: FOREGROUND.r,
                g: FOREGROUND.g,
                b: FOREGROUND.b,
                a: alpha,
            };
            macroquad::text::draw_text(message, x_pos, y_pos, TEXTSIZE, text_color);
            //reset the game after elapsed time < 0.0
            let elapsed_time = text_fadeout_time - macroquad::time::get_time();
            if elapsed_time < 0.0 {
                //reset the game bord/field:
                field = [[0; 3]; 3];
                announcer_message = None;
                text_fadeout_time = 0.0;
            }
        }
        //draw score at the bottom center
        let score_text = &format!(
            "You: {}|Bot: {}| Draws: {}",
            human_score, ai_score, draw_count
        );
        let textdimensions = macroquad::text::measure_text(score_text, None, 1, TEXTSIZE);
        let text_width = textdimensions.width;
        let text_height = textdimensions.height;
        let screen_center_x = macroquad::window::screen_width() * 0.5;
        let x_pos = screen_center_x - text_width * 0.5;
        let y_pos = macroquad::window::screen_height() - text_height * 1.5;
        macroquad::text::draw_text(score_text, x_pos, y_pos, TEXTSIZE * 0.75, TEXTCOLOR);
        macroquad::window::next_frame().await
    }
}
