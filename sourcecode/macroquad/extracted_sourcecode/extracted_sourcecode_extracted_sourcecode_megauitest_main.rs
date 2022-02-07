extern crate macroquad;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut megauitest = macroquad::megaui::Ui::new();
    loop {
        macroquad::clear_background(macroquad::BLACK);
        if megauitest.button(None, "klick me") {
            println!("clicked");
        }

        macroquad::next_frame().await
    }
}
