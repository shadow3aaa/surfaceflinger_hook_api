mod refresh_rate;

use std::{
    fs::File,
    io::prelude::*,
    path::Path,
    sync::{atomic::AtomicU32, Arc},
    thread,
    time::Duration,
};

use crate::{
    error::{Error, Result},
    JankLevel, JankType, API_DIR,
};
use refresh_rate::get_refresh_rate;

pub struct Connection {
    hook_input_pipe: File,
    jank_pipe: File,
    display_fps: Arc<AtomicU32>,
}

impl Connection {
    // 初始化和hook的连接，堵塞
    pub fn init_and_wait(t: JankType) -> Result<Self> {
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

        let display_fps = get_refresh_rate().unwrap_or_default();
        let temp_fps = display_fps;

        let display_fps = Arc::new(AtomicU32::new(display_fps));
        {
            let display_fps = display_fps.clone();
            thread::Builder::new()
                .name("DisplayFpsDumper".into())
                .spawn(move || refresh_rate::dumper(&display_fps))
                .map_err(|_| Error::Other("Failed to start DisplayFpsDumper thread"))?;
        }

        match t {
            JankType::Vsync => writeln!(hook_input_pipe, "{temp_fps}:{temp_fps}:vsync")?,
            JankType::Soft => writeln!(hook_input_pipe, "{temp_fps}:{temp_fps}:soft")?,
        } // 解除surfacrflinger hook的堵塞，初始化

        Ok(Self {
            jank_pipe,
            display_fps,
            hook_input_pipe,
        })
    }

    pub fn recv_jank(&mut self, _target_fps: u32) -> Result<JankLevel> {
        todo!("接受管道jank等级")
    }
}
