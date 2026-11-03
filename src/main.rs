use flake::prelude::*;
use flake_egui::FlakeEgui;

#[flake]
async fn main() {
    let mut gui = FlakeEgui::new();

    loop {
        clear_background(BLACK);
        let ctx = gui.begin();

        egui::Window::new("Debug").show(ctx, |ui| {
            if ui.button("hi").is_pointer_button_down_on() {
                draw_rectangle(0.0, 0.0, 50.0, 50.0, RED)
            }
        });

        gui.end();
        draw_rectangle(0.0, 0.0, 50.0, 50.0, RED);
        end_frame!();
    }
}
