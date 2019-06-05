use super::scene::Scene;
use super::util::*;
use std::vec::Vec;

extern crate image;
extern crate rand;

use rand::Rng;

pub struct Camera {
    position: Vector3,
    direction: Vector3,
    picture: Vec<Color>,
    width: usize,
    height: usize,
    scene: Scene,
}

impl Camera {
    pub fn new(scene: Scene) -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
            picture: Vec::new(),
            width: 0,
            height: 0,
            scene,
        }
    }

    pub fn set_pos(&mut self, new_pos: &Vector3) {
        self.position = *new_pos;
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.height = height;
        self.width = width;
        self.picture.resize((width * height) as usize, Color::default());
    }

    pub fn set_dir(&mut self, direction: Vector3) {
        self.direction = direction;
    }

    // only need to do once
    fn ray_tracing(&mut self) {
        let cx = Vector3::new(0.0, self.width as f64 / self.height as f64, 0.0);
        let cy = Vector3::new(0.0, 0.0, -1.0);
        for j in 0..self.height {
            for i in 0..self.width {
                for sy in 0..2 {
                    for sx in 0..2 {
                        let mut res = Vector3::new(0.0, 0.0, 0.0);
                        for s in 0..10 {
                            let r1: f64 = rand::thread_rng().gen_range(0.0, 2.0);
                            let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
                            let r2: f64 = rand::thread_rng().gen_range(0.0, 2.0);
                            let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
                            let d = cx.mult((i as f64 + (sx as f64 + 0.5 + dx) / 2.0) / self.width as f64 - 0.5)
                                + cy.mult((j as f64 + (sy as f64 + 0.5 + dy) / 2.0) / self.height as f64 - 0.5) + self.direction;
                            let ray = Ray {
                                o: self.position + d.mult(100.0),
                                d: d.normalize(),
                            };
                            // TODO add scene.ray_tracing func here
                        }
                    }
                }
                info!(" line : {}, col : {}", i, j);
            }
        }
    }

    pub fn run(&mut self, times: usize) {
        let buffer: &mut [u8] = &mut [0; 1024 * 768 * 3];

        self.ray_tracing();
        for _ in 0..times {
            // TODO to run a photon tracing func here
        }

        //将结果写入png
        for i in 0..self.width {
            for j in 0..self.height {
                let (r, g, b) = self.picture[j * self.width + i].to_int();
                buffer[(j * self.width + i) * 3] = r;
                buffer[(j * self.width + i) * 3 + 1] = g;
                buffer[(j * self.width + i) * 3 + 2] = b;
            }
        }
        image::save_buffer("result.png", buffer, 1024, 768, image::RGB(32)).unwrap()
    }
}