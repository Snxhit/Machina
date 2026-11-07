use egui::{Context, ViewportId};
use pixels::wgpu::wgc::command;
use winit::{event::WindowEvent, window::Window};
use egui_wgpu::wgpu;

use crate::sim::element::ElementType;

pub struct Gui {
    ctx: Context,
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
}

impl Gui {
    pub fn new(window: &Window, device: &wgpu::Device, output_format: wgpu::TextureFormat) -> Self {
        let ctx = Context::default();

        let state = egui_winit::State::new(
            ctx.clone(),
            ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            window.theme(),
            None,
        );

        let renderer = egui_wgpu::Renderer::new(
            device,
            output_format,
            egui_wgpu::RendererOptions::default(),
        );

        Self {
            ctx,
            state,
            renderer,
        }
    }

    pub fn on_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.state.on_window_event(window, event).consumed
    }

    pub fn wants_pointer_input(&self) -> bool {
        self.ctx.egui_wants_pointer_input()
    }

    pub fn update(&mut self, window: &Window, selected: &mut ElementType) -> egui::FullOutput {
        let raw_input = self.state.take_egui_input(window);

        let mut full_output = self.ctx.run_ui(raw_input, |ui| {
            egui::Panel::left("tool_panel")
                .resizable(false)
                .exact_size(140.0)
                .show(ui, |ui| {
                    ui.heading("Tools");
                    ui.separator();

                    if ui
                        .selectable_label(*selected == ElementType::Sand, "Sand")
                        .clicked() {
                        *selected = ElementType::Sand;
                    }

                    if ui
                        .selectable_label(*selected == ElementType::Water, "Water")
                        .clicked() {
                        *selected = ElementType::Water;
                    }
                });
        });

        let platform_output = std::mem::take(&mut full_output.platform_output);
        self.state.handle_platform_output(window, platform_output);

        full_output
    }

    pub fn paint(
        &mut self,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &pixels::PixelsContext<'_>,
        full_output: egui::FullOutput,
    ) {
        let size = window.inner_size();
        let pixels_per_point = full_output.pixels_per_point;

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer.update_texture(&context.device, &context.queue, *id, image_delta);
        }

        let paint_jobs = self.ctx.tessellate(full_output.shapes, pixels_per_point);

        let command_buffers = self.renderer.update_buffers(&context.device, &context.queue, encoder, &paint_jobs, &screen_descriptor);

        if !command_buffers.is_empty() {
            context.queue.submit(command_buffers);
        }

        {
            let mut render_pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui_render_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: render_target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None 
                })
                .forget_lifetime();

            self.renderer
                .render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }
}