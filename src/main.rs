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
use std::process::exit;

mod chip8;
mod platform;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    platform: Platform
}

const SCALE: f64 = 20.0;
const VIDEO_WIDTH: f64 = 64.0;
const VIDEO_HEIGHT: f64 = 32.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const LINE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        //See Grid
        let grid = grid::Grid{
            cols: 64,
            rows: 32,
            units: SCALE
        };
        let line = Line::new(LINE_COLOR, 0.5);
        let platform = &mut self.platform;


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            grid.draw(&line, &c.draw_state, c.transform, gl);
            // Draw a box rotating around the middle of the screen.
            // rectangle(WHITE, square, transform, gl);
            for (x,y) in grid.cells(){
                let mut col = BLACK;
                if platform.get_gfx()[(y as usize * VIDEO_WIDTH as usize) + x as usize] != 0 {
                    col = WHITE;
                }

                rectangle(col, rectangle::square(x as f64* SCALE, y as f64 * SCALE, SCALE), c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.platform.c8_cycle();
    }

    fn set_input(&mut self, btn: Button, is_pressed: bool){
        match btn {
            Button::Keyboard(key) => {
                let input_code = format!("{:?}", key);
                println!("{}", input_code);
                self.platform.handle_input(&input_code, is_pressed);
            },
            _ => println!("Something else")
        }
    }
}

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [SCALE * VIDEO_WIDTH, SCALE * VIDEO_HEIGHT])
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
    app.platform.open_rom("games/Airplane.ch8");

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
