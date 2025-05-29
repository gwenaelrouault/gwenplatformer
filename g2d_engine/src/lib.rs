use crate::gui::gui::Gui;
use error_iter::ErrorIter;
use image::{imageops::resize, imageops::FilterType, ImageBuffer, Rgba, RgbaImage};
use log::{error, info};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard;
use winit::keyboard::KeyCode::F11;
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub mod gui {
    pub mod framework;
    pub mod gui;
}

pub struct G2dEngine {
    screen_width: u32,
    screen_height: u32,
    playing: bool,
    background: RgbaImage,
}

impl G2dEngine {
    pub fn new(width: u32, height: u32, background: RgbaImage) -> Self {
        Self {
            screen_width: width,
            screen_height: height,
            playing: false,
            background,
        }
    }

    pub fn load_assets(&mut self) {}

    pub fn draw(&self, frame: &mut [u8], default_background: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        if self.playing {} else {
            frame.copy_from_slice(&default_background);
        }
    }

    pub fn update(&self) {}

    pub fn run(&mut self, gui: Box<dyn Gui>) -> Result<(), Error> {
        let mut is_fullscreen = false;
        let event_loop = EventLoop::new().unwrap();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(self.screen_width as f64, self.screen_height as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels + egui")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(true)
                .with_fullscreen(None)
                .build(&event_loop)
                .unwrap()
        };
        let mut size = window.inner_size();
        let mut resized_image = resize(
            &self.background,
            size.width,
            size.height,
            FilterType::Nearest,
        );
        let (mut pixels, mut framework) = {
            let window_size = window.inner_size();
            let scale_factor = window.scale_factor() as f32;
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            let pixels = Pixels::new(self.screen_width, self.screen_height, surface_texture)?;
            let framework = gui::framework::Framework::new(
                &event_loop,
                window_size.width,
                window_size.height,
                scale_factor,
                &pixels,
                gui,
            );
            (pixels, framework)
        };
        let res = event_loop.run(|event, elwt| {
            if input.update(&event) {
                if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                    elwt.exit();
                    return;
                }
                if let Some(scale_factor) = input.scale_factor() {
                    framework.scale_factor(scale_factor);
                }
                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        elwt.exit();
                        return;
                    }
                    framework.resize(size.width, size.height);
                }
                self.update();
                window.request_redraw();
            }
            match event {
                Event::WindowEvent {
                    event:
                    WindowEvent::KeyboardInput {
                        event: KeyEvent { physical_key, state, .. },
                        ..
                    },
                    ..
                } => if physical_key == PhysicalKey::Code(KeyCode::F11) && state == ElementState::Pressed {
                    info!("F11");
                    is_fullscreen = !is_fullscreen;
                    if is_fullscreen {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    } else {
                        window.set_fullscreen(None);
                    }
                },

                // Draw the current frame
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Draw the world
                    self.draw(pixels.frame_mut(), &resized_image);
                    // Prepare egui
                    framework.prepare(&window);
                    // Render everything together
                    let render_result = pixels.render_with(|encoder, render_target, context| {
                        // Render the world texture
                        context.scaling_renderer.render(encoder, render_target);
                        // Render egui
                        framework.render(encoder, render_target, context);
                        Ok(())
                    });
                    if let Err(err) = render_result {
                        log_error("pixels.render", err);
                        elwt.exit();
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    size = new_size;
                    framework.resize(size.width, size.height);
                    pixels.resize_surface(size.width, size.height);
                    pixels.resize_buffer(size.width, size.height);
                    resized_image = resize(
                        &self.background,
                        size.width,
                        size.height,
                        FilterType::Nearest,
                    );
                }
                Event::WindowEvent { event, .. } => {
                    framework.handle_event(&window, &event);
                }
                _ => (),
            }
        });
        res.map_err(|e| Error::UserDefined(Box::new(e)))
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
