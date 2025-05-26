use error_iter::ErrorIter;
use image::RgbaImage;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::gui::gui::Gui;

pub mod gui {
    pub mod gui;
    pub mod framework;
}

pub struct G2dEngine {
    screen_width: u32,
    screen_height: u32,
    playing : bool,
    background: Vec<u8>,
}

impl G2dEngine {
    pub fn new(width: u32, height: u32, background : RgbaImage) -> Self {
        let resized = image::imageops::resize(
            &background,
            width,
            height,
            image::imageops::FilterType::Lanczos3,
        );
        let img_pixels = resized.into_raw();
        Self {
            screen_width: width,
            screen_height: height,
            playing : false,
            background : img_pixels
        }
    }

    pub fn load_assets(&mut self) {}

    pub fn draw(&self, frame: &mut [u8]) {
        if self.playing {
            
        }
        else {
            self.draw_background(frame);
        }
    }

    pub fn update(&self) {}
    
    fn draw_background(&self, frame: &mut [u8]) {
        frame.copy_from_slice(&self.background);
    }

    pub fn run(&mut self, gui: Box<dyn Gui>) -> Result<(), Error> {
        let event_loop = EventLoop::new().unwrap();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(self.screen_width as f64, self.screen_height as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels + egui")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };
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
                gui
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
                // Draw the current frame
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Draw the world
                    self.draw(pixels.frame_mut());

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
