extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent, ReleaseEvent};
use crate::platform::Platform;

mod chip8;
mod platform;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    platform: Platform
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn set_input(&mut self, btn: Button, is_pressed: bool){
        match btn {
            Button::Keyboard(key) => {
                let input_code = format!("{:?}", key);
                self.platform.handle_input(&input_code, is_pressed);
            },
            _ => println!("Something else")
        }
    }
}

fn main() {


    println!("Hello, world!");

    let c8 = chip8::new_chip8();
    c8.chip8_says_hello();
    c8.print_registers();
    c8.print_stack();
    c8.print_memory();

    let mut platform = platform::new_platform();
    platform.open_rom("test_opcode.ch8");
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        platform: platform::new_platform()
    };

    let mut settings = EventSettings::new();
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {

        //Receive input
        if let Some(btn) = e.press_args() {
            // println!("Pressed keyboard key '{:?}'", key);
            app.set_input(btn, true);
        };
        if let Some(btn) = e.release_args() {
            // println!("Pressed keyboard key '{:?}'", key);
            app.set_input(btn, false);
        };
        //update chip8
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        //Render the gfx
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }




}
