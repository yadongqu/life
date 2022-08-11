extern crate rand;
use std::{cell, thread, time, vec};

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
struct Life {
    world: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        let mut random = rand::thread_rng();
        let mut world = vec![vec![0u8; height]; width];

        // world[0][1] = 1;
        // world[1][0] = 1;
        // world[1][1] = 1;

        for i in 0..width {
            for j in 0..height {
                let alive = random.gen_bool(0.5);
                if alive {
                    world[i][j] = 1;
                }
            }
        }

        Life {
            world,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let mut new_world = vec![vec![0u8; self.height]; self.width];
        for i in 0..self.width {
            for j in 0..self.height {
                let mut count = 0u32;
                if i > 0 && self.world[i - 1][j] == 1 {
                    count += 1;
                }
                if i < self.width - 1 && self.world[i + 1][j] == 1 {
                    count += 1;
                }
                if j > 0 && self.world[i][j - 1] == 1 {
                    count += 1;
                }
                if j < self.height - 1 && self.world[i][j + 1] == 1 {
                    count += 1;
                }
                if i > 0 && j > 0 && self.world[i - 1][j - 1] == 1 {
                    count += 1;
                }
                if i > 0 && j < self.height - 1 && self.world[i - 1][j + 1] == 1 {
                    count += 1;
                }
                if i < self.width - 1 && j > 0 && self.world[i + 1][j - 1] == 1 {
                    count += 1;
                }
                if i < self.width - 1 && j < self.height - 1 && self.world[i + 1][j + 1] == 1 {
                    count += 1;
                }
                new_world[i][j] = 0;

                if (count < 2 || count > 3) && self.world[i][j] == 1 {
                    new_world[i][j] = 0;
                }
                if (count == 2 || count == 3) && self.world[i][j] == 1 {
                    new_world[i][j] = 1;
                }
                if count == 3 && self.world[i][j] == 0 {
                    new_world[i][j] = 1;
                }
            }
        }
        self.world = new_world;
    }

    pub fn render(&self, framebuffer: &mut Vec<u32>) {
        // a trick to clear the screen.
        // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed

        let row_len = self.height * 10;
        for i in 0..self.width {
            for j in 0..self.height {
                let alive = self.world[i][j];
                let x = i * 10;
                let y = j * 10;

                for s in 0..10 {
                    for t in 0..10 {
                        let index = (s + x) * row_len + y + t;
                        if alive == 1 {
                            framebuffer[index] = 255;
                        } else {
                            framebuffer[index] = 0;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut life = Life::new(80, 80);
    let second = time::Duration::from_millis(300);
    let cell_size = 10usize;
    let window_width = life.width * cell_size;
    let window_height = life.height * cell_size;
    let mut window = Window::new(
        "Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut framebuffer = vec![0; life.width * life.height * cell_size * cell_size];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        life.update();
        life.render(&mut framebuffer);
        window
            .update_with_buffer(&framebuffer, window_width, window_height)
            .unwrap();
        thread::sleep(second);
    }
}
