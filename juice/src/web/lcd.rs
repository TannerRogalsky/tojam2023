#![allow(unused)]

use padme_core::{Pixel, Screen, FRAME_HEIGHT, FRAME_WIDTH};

const FRAMEBUFFER_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * 4;

pub struct Lcd {
    framebuffer: [u8; FRAMEBUFFER_SIZE],
}

impl Lcd {
    pub const fn new() -> Self {
        Self {
            framebuffer: [0xFFu8; FRAMEBUFFER_SIZE],
        }
    }

    pub fn framebuffer(&self) -> *const u8 {
        &self.framebuffer as *const u8
    }

    pub fn clear(&mut self) {
        self.framebuffer.iter_mut().for_each(|p| *p = 0);
    }
}

impl Screen for Lcd {
    fn set_pixel(&mut self, px: &Pixel, x: u8, y: u8) {
        let i = (x as usize + y as usize * FRAME_WIDTH) * 4;
        self.framebuffer[i] = px.r;
        self.framebuffer[i + 1] = px.g;
        self.framebuffer[i + 2] = px.b;
        self.framebuffer[i + 3] = px.a;
    }

    fn update(&mut self) {}
}
