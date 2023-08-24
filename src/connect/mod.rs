mod input;

use std::{
    fs::File,
    io::prelude::*,
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use crate::{
    error::{Error, Result},
    JankLevel, JankType, API_DIR,
};

pub struct Connection {
    jank_pipe: File,
    sx: Sender<(u32, JankType)>,
}

impl Connection {
    // 初始化和hook的连接，堵塞
    pub fn init_and_wait(_t: JankType) -> Result<Self> {
        let hook_input_path = Path::new(API_DIR).join("input");
        let jank_path = Path::new(API_DIR).join("jank");

        loop {
            if hook_input_path.exists() && jank_path.exists() {
                break;
            }

            thread::sleep(Duration::from_secs(1));
        } // 等待hook创建管道

        let mut hook_input_pipe = File::open(&hook_input_path)?;
        let jank_pipe = File::open(&jank_path)?;

        let (sx, rx) = mpsc::channel();

        thread::Builder::new()
            .name("StatusUpdater".into())
            .spawn(move || input::updater(&rx, &mut hook_input_pipe))
            .map_err(|_| Error::Other("Failed to start updater thread"))?;

        Ok(Self { jank_pipe, sx })
    }

    pub fn set_input(&self, t: u32, j: JankType) -> Result<()> {
        self.sx
            .send((t, j))
            .map_err(|_| Error::Other("Failed to send input"))?;
        Ok(())
    }

    pub fn recv_jank(&mut self) -> Result<JankLevel> {
        let mut r = String::new();
        self.jank_pipe.read_to_string(&mut r)?;

        let level: u32 = r
            .trim()
            .lines()
            .last()
            .and_then(|l| l.trim().parse().ok())
            .ok_or(Error::Other("Failed to parse jank-level"))?;

        Ok(JankLevel(level))
    }
}
