use std::cell::RefCell;
use std::rc::Rc;
use flake::prelude::*;
pub use egui;

pub struct FlakeEgui {
    ctx: egui::Context,
    state: egui_winit::State,
    renderer: Rc<RefCell<egui_wgpu::Renderer>>,
    pending: Rc<RefCell<Option<(Vec<egui::ClippedPrimitive>, f32)>>>,
}

impl FlakeEgui {
    pub fn new() -> Self {
        let window = get_window();
        let ctx = egui::Context::default();
        let state = egui_winit::State::new(
            ctx.clone(), egui::ViewportId::ROOT, &*window,
            Some(window.scale_factor() as f32), None, None,
        );
        let format = get_surface_config().lock().unwrap().format;
        let renderer = Rc::new(RefCell::new(
            egui_wgpu::Renderer::new(&get_device(), format, Default::default()),
        ));
        let pending: Rc<RefCell<Option<(Vec<egui::ClippedPrimitive>, f32)>>> =
            Rc::new(RefCell::new(None));

        let (r, p) = (renderer.clone(), pending.clone());
        add_post_render_hook(move |device, queue, encoder, view, size| {
            let Some((jobs, ppp)) = p.borrow_mut().take() else { return };
            let sd = egui_wgpu::ScreenDescriptor { size_in_pixels: [size.0, size.1], pixels_per_point: ppp };
            let mut r = r.borrow_mut();
            r.update_buffers(device, queue, encoder, &jobs, &sd);
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view, resolve_target: None, depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                ..Default::default()
            });
            r.render(&mut pass.forget_lifetime(), &jobs, &sd);
        });

        Self { ctx, state, renderer, pending }
    }

    /// Call at the start of each frame
    pub fn begin(&mut self) -> &egui::Context {
        let window = get_window();
        for e in get_window_events() {
            let _ = self.state.on_window_event(&window, &e);
        }
        self.ctx.begin_pass(self.state.take_egui_input(&window));
        &self.ctx
    }

    /// Call at the end of each frame
    pub fn end(&mut self) {
        let mut out = self.ctx.end_pass();
        self.state.handle_platform_output(&get_window(), out.platform_output);

        let (device, queue) = (get_device(), get_queue());
        let mut r = self.renderer.borrow_mut();
        let mut td = std::mem::take(&mut out.textures_delta);
        for (id, deltas) in td.set.drain() {
            for delta in &deltas {
                r.update_texture(&device, &queue, id, delta);
            }
        }
        for id in td.free.drain() {
            r.free_texture(&id);
        }
        for id in &out.textures_delta.free { r.free_texture(id); }

        let jobs = self.ctx.tessellate(out.shapes, out.pixels_per_point);
        *self.pending.borrow_mut() = Some((jobs, out.pixels_per_point));
    }

    /// True if egui is currently interested in the pointer (mouse or touch).
    ///
    // Could be the pointer is hovering over a crate::Window or the user is dragging a widget.
    // If false, the pointer is outside any egui area and so you may be interested in what it is doing (e.g. controlling your game).
    // Returns false if a drag started outside egui and then moved over an egui area.
    pub fn wants_pointer(&self) -> bool { self.ctx.egui_wants_pointer_input() }

    /// If true, egui is currently listening on text input (e.g. typing text in a `TextEdit`).
    pub fn wants_keyboard(&self) -> bool { self.ctx.egui_wants_keyboard_input() }
}