/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
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
    sx: Sender<(Option<u32>, JankType)>,
}

impl Connection {
    /// Initialize the connection, block until the connection is successful
    ///
    /// # Errors
    ///
    /// Failed to open pipe (sugg: Check if you have root access)
    pub fn init_and_wait() -> Result<Self> {
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

    /// Set `target_fps` and settlement point for calculating jank
    ///
    /// Use `display_refresh_rate` when `target_fps` is set to None
    ///
    /// # Errors
    ///
    /// Failed to send message to setter thread
    pub fn set_input(&self, t: Option<u32>, j: JankType) -> Result<()> {
        self.sx
            .send((t, j))
            .map_err(|_| Error::Other("Failed to send input data"))?;
        Ok(())
    }

    /// Blocking receiving the latest jank
    ///
    /// # Errors
    ///
    /// Failed to open pipe / Failed to parse jank-level
    pub fn recv(&self) -> Result<JankLevel> {
        let r = fs::read_to_string(&self.jank_pipe)?;

        let level: u32 = r
            .trim()
            .lines()
            .last()
            .and_then(|l| l.trim().parse().ok())
            .ok_or(Error::NamedPipe)?;

        Ok(JankLevel(level))
    }

    /// Receiving the latest jank, no blocking
    ///
    /// # Errors
    ///
    /// No jank-messge / Failed to open pipe / Failed to parse jank-level
    pub fn try_recv(&self) -> Result<JankLevel> {
        let mut p = named_pipe::open_read(&self.jank_pipe)?;

        let mut r = String::new();
        p.read_to_string(&mut r)?;

        let l = r
            .trim()
            .lines()
            .last()
            .ok_or(Error::Other("No jank-message finded"))?;

        let level: u32 = l.trim().parse().map_err(|_| Error::NamedPipe)?;

        Ok(JankLevel(level))
    }
}
