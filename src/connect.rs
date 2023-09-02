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
use std::{
    fs::{self},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use crate::{
    error::{Error, Result},
    JankLevel, API_DIR,
};

/// ipc with surfaceflinger
pub struct Connection {
    jank_pipe: PathBuf,
    input_pipe: PathBuf,
}

impl Connection {
    /// Initialize the connection, block until the connection is successful
    ///
    /// # Errors
    ///
    /// Failed to open pipe (sugg: Check if you have root access)
    pub fn init_and_wait() -> Result<Self> {
        let input_path = Path::new(API_DIR).join("input");
        let jank_path = Path::new(API_DIR).join("jank");

        loop {
            if input_path.exists() && jank_path.exists() {
                break;
            }

            thread::sleep(Duration::from_secs(1));
        }

        let s = Self {
            jank_pipe: jank_path,
            input_pipe: input_path,
        };

        s.set_input(None)?;
        let _ = s.recv();

        Ok(s)
    }

    /// Set inputs for calculating jank
    ///
    /// # Errors
    ///
    /// io error
    pub fn set_input(&self, t: Option<(u32, Duration)>) -> Result<()> {
        let message = t.map_or_else(|| "none".into(), |(t, d)| format!("{t}:{}", d.as_nanos()));
        fs::write(&self.input_pipe, message)?;
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
}
