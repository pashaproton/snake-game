use std::collections::LinkedList;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;

pub struct ConfigWindow {
    pub width: u32,
    pub height: u32,
}

impl ConfigWindow {
    fn new(width: u32, height: u32) -> ConfigWindow {
        ConfigWindow {
            width,
            height,
        }
    }

    pub fn get_window_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }
}

pub struct ConfigSnake {
    size: i8,
    pub speed: u64,
    position: (i32, i32)
}

impl ConfigSnake {
    fn new(size: i8, speed: u64, position: (i32, i32)) -> ConfigSnake {
        ConfigSnake {
            size,
            speed,
            position,
        }
    }

    pub fn get_size(&self) -> Vec<(i32, i32)> {
        let mut i: i8 = 0;
        let mut result: Vec<(i32, i32)> = Vec::new();

        while i < self.size {
            result.push(((i as i32) + self.position.0, self.position.1));
            i += 1;
        }

        result
    }
 }

pub struct Config {
    pub window: ConfigWindow,
    pub snake: ConfigSnake,
}

impl Config {
    pub fn new(window: ConfigWindow, snake: ConfigSnake) -> Config {
        Config {
            window,
            snake,
        }
    }
}

pub fn get_config() -> Config {
    let config_data = tsu::toml_from_path("config.toml");
    let config_data_window = config_data.get("window")
        .expect("Should read from config_data property window");

    let width = config_data_window.get("width")
        .expect("Should read from config_data_window property width")
        .as_integer()
        .expect("Should return width as integer") as u32;

    let height: u32 = config_data_window.get("height")
        .expect("Should read from config_data_window property height")
        .as_integer()
        .expect("Should return height as integer") as u32;
    
    let window = ConfigWindow::new(width, height);

    let config_data_snake = config_data.get("snake")
        .expect("Should read from config_data property snake");

    let size = config_data_snake.get("size")
        .expect("Should read from config_data_snake property size")
        .as_integer()
        .expect("Should return size as integer") as i8;
    let speed = 6;
    let position: (i32, i32) = (0, 0); // ((width / 2) as i32, (height / 2) as i32);
    let snake = ConfigSnake::new(size, speed, position);

    Config::new(window, snake)
}

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Right, Left, Up, Down
}

#[derive(Debug)]
pub enum SnakeBehaviour {
    Eat,
    Move,
}

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub window: ConfigWindow,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        let background_color: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        self.gl.draw(arg.viewport(), |_c: Context, gl: &mut GlGraphics| {
            graphics::clear(background_color, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) -> Result<SnakeBehaviour, &'static str> {
        if self.food.is_eaten(&self.snake) {
            self.food.update(&self.window);
            self.snake.update(&self.window, true)
        } else {
            self.snake.update(&self.window, false)
        }
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction: Direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub dir: Direction,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let snake_color: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 10) as f64,
                    (y * 10) as f64,
                    10_f64
                )
            })
            .collect();

        

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter().for_each(|square| {
                graphics::rectangle(snake_color, square, transform, gl);
            });
        });
    }

    fn update(&mut self, window: &ConfigWindow, grow: bool) -> Result<SnakeBehaviour, &'static str> {
        let mut new_head = (*self.body.front().expect("Snake has no body.")).clone();

        match self.dir {
            Direction::Left => {
                if new_head.0 <= 0 {
                    return Err("Game over!")
                }

                new_head.0 -= 1;
            },
            Direction::Right => {
                if new_head.0 >= window.width as i32 {
                    return Err("Game over!")
                }

                new_head.0 += 1;
            },
            Direction::Up => {
                if new_head.1 <= 0 {
                    return Err("Game over!")
                }

                new_head.1 -= 1;
            },
            Direction::Down => {
                if new_head.1 >= window.height as i32 {
                    return Err("Game over!")
                }

                new_head.1 += 1;
            },
        }

        self.body.push_front(new_head);

        if grow {
            return Ok(SnakeBehaviour::Eat)
        }

        self.body.pop_back().unwrap();
        Ok(SnakeBehaviour::Move)
    }
}

pub struct Food {
    pub position: (i32, i32),
}

impl Food {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let food_color: [f32; 4] = [1.0, 0.7, 0.7, 1.0];

        let square = graphics::rectangle::square(
            (self.position.0 * 10) as f64,
            (self.position.1 * 10) as f64,
            10_f64
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::circle_arc(food_color, 5_f64, 1.0, 0.9, square, transform, gl);
            // graphics::rectangle(food_color, square, transform, gl);
        });
    }

    fn update(&mut self, window: &ConfigWindow) {
        let mut rng = rand::thread_rng();

        self.position.0 = rng.gen_range(0..(window.width / 10) as i32);
        self.position.1 = rng.gen_range(0..(window.height / 10) as i32);
    }

    fn is_eaten(&self, snake: &Snake) -> bool {
        let head = snake.body.front().unwrap();
        self.position.0 == head.0 && self.position.1 == head.1
    }
}
