#![allow(unused)]

use padme_core::SerialOutput;

pub struct SerialConsole;

impl SerialConsole {
    pub fn new() -> Self {
        Self
    }
}

impl SerialOutput for SerialConsole {
    fn putchar(&mut self, c: u8) {}
}
