use std::{fs::File, str::FromStr, io::Write};

use chrono::prelude::*;

pub struct Log {
    file: File,
}

impl Log {
    pub fn new() -> Self {
        let t = Local::now();
        let file = File::create(format!("logs/log-{}.txt", t.format("%Y-%m-%d-%H-%M-%S"))).unwrap(); 
        Self { file }
    }

    pub fn log(&mut self, buf: &[u8]) {
        self.file.write_all(buf).unwrap();
        self.file.write_all(b"\n").unwrap();
    }
}