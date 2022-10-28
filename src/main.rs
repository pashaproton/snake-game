extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::Size;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use snake::{Direction, get_config, Game, Snake, Food, SnakeBehaviour};
use std::iter::FromIterator;
use std::collections::LinkedList;

fn main() {
    let opengl = OpenGL::V4_5;
    let config = get_config();
    let mut speed = config.snake.speed;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        Size::from(config.window.get_window_size()),
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new()).ups(speed);

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(config.snake.get_size().into_iter()),
            dir: Direction::Right,
        },
        food: Food { position: (10, 10) },
        window: config.window,
    };

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if e.update_args().is_some() {
            match game.update() {
                Ok(behaviour) => {
                    if let SnakeBehaviour::Eat = behaviour {
                        speed += 1;
                        events.set_ups(speed);
                    }
                },
                Err(error) => {
                    println!("{}", error);
                    break;
                },
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
