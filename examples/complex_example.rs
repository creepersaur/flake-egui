use flake::prelude::*;
use flake_egui::FlakeEgui;

#[flake]
async fn main() {
    let mut gui = FlakeEgui::new();

    // UI state lives outside the loop
    let mut name = String::from("John Doe");
    let mut speed = 5.0_f32;
    let mut show_grid = true;
    let mut color = [0.2_f32, 0.6, 1.0];
    let mut counter = 0;
    let mut log: Vec<String> = vec![];

    loop {
        clear_background(BLACK);
        let ctx = gui.begin();

        ctx.root_ui(|ui| {
            // Top menu bar
            egui::Panel::top("menu").show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Clear log").clicked() {
                            log.clear();
                            ui.close();
                        }
                    });
                    ui.label(format!("FPS: {}", get_fps()));
                });
            });
        });

        // Settings window
        egui::Window::new("Settings").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut name);
            });

            ui.add(egui::Slider::new(&mut speed, 0.0..=20.0).text("Speed"));
            ui.checkbox(&mut show_grid, "Show grid");
            ui.color_edit_button_rgb(&mut color);

            ui.separator();

            if ui.button(format!("Clicked {counter} times")).clicked() {
                counter += 1;
                log.push(format!("{name} clicked (speed {speed:.1})"));
            }
        });

        // Log window
        egui::Window::new("Log")
            .default_pos([300.0, 40.0])
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for line in &log {
                            ui.label(line);
                        }
                    });
            });

        gui.end();

        // Use the UI state in your own drawing
        if show_grid {
            for i in 0..80 {
                draw_line(
                    (i * 10) as f32,
                    0.0,
                    (i * 10) as f32,
                    600.0,
                    1.0,
                    Color::from_vec3(color.into()),
                );

                draw_line(
                    0.0,
                    (i * 10) as f32,
                    800.0,
                    (i * 10) as f32,
                    1.0,
                    Color::from_vec3(color.into()),
                );
            }
        }

        end_frame!();
    }
}
