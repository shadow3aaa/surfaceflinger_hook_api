mod input;

use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use unix_named_pipe as named_pipe;

use crate::{
    error::{Error, Result},
    JankLevel, JankType, API_DIR,
};

/// ipc with surfaceflinger
pub struct Connection {
    jank_pipe: PathBuf,
    sx: Sender<(u32, JankType)>,
}

impl Connection {
    /// Initialize the connection, block until the connection is successful
    ///
    /// # Error
    ///
    /// Fail to open pipe (sugg: Check if you have root access)
    pub fn init_and_wait(_t: JankType) -> Result<Self> {
        let hook_input_path = Path::new(API_DIR).join("input");
        let jank_path = Path::new(API_DIR).join("jank");

        loop {
            if hook_input_path.exists() && jank_path.exists() {
                break;
            }

            thread::sleep(Duration::from_secs(1));
        } // Wait until surfaceflinger created named pipe

        let mut hook_input_pipe = File::open(&hook_input_path)?;

        let (sx, rx) = mpsc::channel();

        thread::Builder::new()
            .name("StatusUpdater".into())
            .spawn(move || input::updater(&rx, &mut hook_input_pipe))
            .map_err(|_| Error::Other("Failed to start updater thread"))?;

        Ok(Self {
            jank_pipe: jank_path,
            sx,
        })
    }

    /// Set target_fps and settlement point for calculating jank
    ///
    /// # Error
    ///
    /// Fail to send message to setter thread
    pub fn set_input(&self, t: u32, j: JankType) -> Result<()> {
        self.sx
            .send((t, j))
            .map_err(|_| Error::Other("Failed to send input"))?;
        Ok(())
    }

    /// Blocking receiving the latest jank
    pub fn recv(&self) -> Result<JankLevel> {
        let r = fs::read_to_string(&self.jank_pipe)?;

        let level: u32 = r
            .trim()
            .lines()
            .last()
            .and_then(|l| l.trim().parse().ok())
            .ok_or(Error::Other("Failed to parse jank-level"))?;

        Ok(JankLevel(level))
    }

    pub fn try_recv(&self) -> Result<JankLevel> {
        let mut p = named_pipe::open_read(&self.jank_pipe)?;

        let mut r = String::new();
        p.read_to_string(&mut r)?;

        let level: u32 = r
            .trim()
            .lines()
            .last()
            .and_then(|l| l.trim().parse().ok())
            .ok_or(Error::Other("Failed to parse jank-level"))?;

        Ok(JankLevel(level))
    }
}
