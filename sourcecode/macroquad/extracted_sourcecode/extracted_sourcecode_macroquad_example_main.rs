use macroquad::*;

fn testfn(x: f32)->f32{
    x*x
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = 0.0;
    const IMAGESIZE: u16 = 128;
    let mut testimage = Image::gen_image_color(IMAGESIZE, IMAGESIZE, Color::new(1.0, 0.0, 1.0, 1.0));
    //lets create a gradient:
    for (i, color) in testimage.get_image_data().iter_mut().enumerate(){
        let x = i as u16%IMAGESIZE;
        let y = i as u16/IMAGESIZE;
        let x_scaled = x as f32/IMAGESIZE as f32;
        let y_scaled = y as f32/IMAGESIZE as f32;
        let grayscale_value = x_scaled*y_scaled;
        *color = Color::new(grayscale_value, grayscale_value, grayscale_value, 1.0);
    }
    let texture = load_texture_from_image(&testimage);
    loop {
        clear_background(RED);
 
        draw_line(x, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text(&format!("x={}", x), 20.0, 20.0, 20.0, DARKGRAY);
        draw_texture(texture, 0.0, 0.0, Color::new(1.0, 1.0, 1.0, 0.5));
        if macroquad::is_key_down(macroquad::KeyCode::A){
            x -= 0.5;
        }else if macroquad::is_key_down(macroquad::KeyCode::D){
            x += 0.5;
            //x = testfn(x);
        }else if macroquad::is_key_down(macroquad::KeyCode::S){
            x = 0.0;
        }
        if macroquad::is_key_down(macroquad::KeyCode::Q){
            break;
        }
        next_frame().await
    }
}
