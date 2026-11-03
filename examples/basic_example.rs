use flake::prelude::*;
use flake_egui::{FlakeEgui, egui};

#[flake]
async fn main() {
    let mut gui = FlakeEgui::new();

    loop {
        clear_background(BLACK);
        let ctx = gui.begin();

        egui::Window::new("Debug").show(ctx, |ui| {
            ui.label("Hello World")
        });

        gui.end();
        end_frame!()
    }
}