# Flake EGUI

`egui` support for `flake-engine`, an immediate mode game framework made to simplify development.
Built on winit and wgpu.

If you already have a flake project, just run this

```sql
cargo add flake-egui
```

## Getting Started

1. Create a project using `cargo add flake-engine flake-egui`
2. Write the following code for a basic flake project:
```rust
use flake::prelude::*;

#[flake]
async fn main() {
    loop {
        end_frame!()
    }
}
```
3. Include the following on the top of your project:
```rust
use flake_egui::{FlakeEgui, egui};
```
4. Create a mutable `FlakeEgui` and make a Context using `gui.begin()`. Then use that context to make ui.
Remember to call `gui.end()` before ending the frame.
```rust
use flake::prelude::*;
use flake_egui::FlakeEgui;

#[flake]
async fn main() {
    let mut gui = FlakeEgui::new();

    loop {
        clear_background(BLACK);
        let ctx = gui.begin();

        egui::Window::new("Debug").show(ctx, |ui| {
            ui.label("Hello World");
        });

        gui.end();     // UI will not be shown if end() isn't called
        end_frame!();
    }
}
```

## Documentation

```rust
// Creates an egui context that can be used for showing windows and panels
gui.begin() -> &Context;

// Call at the end of the frame. Draws the windows and egui output.
gui.end();

// True if egui is currently interested in the pointer (mouse or touch).
gui.wants_pointer() -> bool;

// If true, egui is currently listening on text input (e.g. typing text in a TextEdit).
gui.wants_keyboard() -> bool;
```

## Examples

Check `examples/` in the repo for more information.