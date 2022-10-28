pub struct Window {
    pub width: u32,
    pub height: u32,
}

impl Window {
    fn new(width: u32, height: u32) -> Window {
        Window {
            width,
            height,
        }
    }

    pub fn get_window_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }
}

pub struct Snake {
    size: i8,
    position: (i32, i32)
}

impl Snake {
    fn new(size: i8, position: (i32, i32)) -> Snake {
        Snake {
            size,
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
    pub window: Window,
    pub snake: Snake,
}

impl Config {
    pub fn new(window: Window, snake: Snake) -> Config {
        Config {
            window,
            snake,
        }
    }
}

pub fn get_config() -> Config {
    let size: i8 = 4;
    let width: u32 = 800;
    let height: u32 = 600;
    
    let window: Window = Window::new(width, height);

    let position: (i32, i32) = (0, 0); // ((width / 2) as i32, (height / 2) as i32);
    let snake: Snake = Snake::new(size, position);

    Config::new(window, snake)
}
