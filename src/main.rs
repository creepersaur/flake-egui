use flake::prelude::*;
use flake_egui::{egui, FlakeEgui};

#[flake]
async fn main() {
    let mut gui = FlakeEgui::new();

    // UI state living outside the loop
    let mut show_grid = true;
    let mut rect_pos = [200.0_f32, 150.0_f32];
    let mut rect_size = [80.0_f32, 80.0_f32];
    let mut color = [1.0_f32, 0.3, 0.3]; // Default RED-ish
    let mut counter = 0;

    loop {
        clear_background(Color::from_rgb8(15, 16, 19));

        let ctx = gui.begin();

        // Top menu bar using root_ui
        ctx.root_ui(|ui| {
            egui::Panel::top("menu").show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Reset Position").clicked() {
                            rect_pos = [200.0, 150.0];
                        }
                    });
                    ui.label(format!("FPS: {:.1}", get_fps()));
                });
            });
        });

        // Inspector/Settings Window to control the rectangle
        egui::Window::new("Viewport Controls").show(ctx, |ui| {
            ui.checkbox(&mut show_grid, "Show grid");
            ui.separator();

            ui.label("Rectangle Controls:");
            ui.add(egui::Slider::new(&mut rect_pos[0], 0.0..=600.0).text("Pos X"));
            ui.add(egui::Slider::new(&mut rect_pos[1], 0.0..=400.0).text("Pos Y"));
            ui.add(egui::Slider::new(&mut rect_size[0], 10.0..=200.0).text("Width"));
            ui.add(egui::Slider::new(&mut rect_size[1], 10.0..=200.0).text("Height"));

            ui.color_edit_button_rgb(&mut color);

            ui.separator();

            if ui.button(format!("Click count: {counter}")).clicked() {
                counter += 1;
            }
        });

        gui.end();

        // --- GLOBAL RENDERING PASS AFTER GUI ---
        // 1. Draw grid lines if enabled
        if show_grid {
            for i in 0..80 {
                let pos = (i * 15) as f32;
                draw_line(pos, 0.0, pos, 600.0, 1.0, Color::from_rgba8(35, 37, 43, 100));
                draw_line(0.0, pos, 800.0, pos, 1.0, Color::from_rgba8(35, 37, 43, 100));
            }
        }

        // 2. Draw your global shape using state modified by the egui window!
        draw_rectangle(
            rect_pos[0],
            rect_pos[1],
            rect_size[0],
            rect_size[1],
            Color::from_vec3(color.into())
        );

        end_frame!();
    }
}