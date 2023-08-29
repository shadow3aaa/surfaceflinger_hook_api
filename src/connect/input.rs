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
    fs,
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc::Receiver,
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{error::Result, JankType};

pub fn updater(rx: &Receiver<(Option<u32>, JankType)>, p: &Path, df: &Arc<AtomicU32>) {
    let display_fps = get_refresh_rate().unwrap_or_default();
    let mut status = (display_fps, display_fps, JankType::Soft);

    df.store(display_fps, Ordering::Release);

    let (t, d, j) = status;
    let _ = write_input(p, t, d, j);

    loop {
        let mut temp_status = status;

        temp_status.1 = get_refresh_rate().unwrap_or_default();

        if let Ok((t, j)) = rx.try_recv() {
            let t = t.unwrap_or_else(|| get_refresh_rate().unwrap_or_default());
            temp_status.0 = t;
            temp_status.2 = j;
        }

        if status != temp_status {
            status = temp_status;

            let (t, d, j) = status;
            let _ = write_input(p, t, d, j);

            df.store(d, Ordering::Release);
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn write_input(p: &Path, t: u32, d: u32, j: JankType) -> Result<()> {
    let message = match j {
        JankType::Vsync => format!("{t}:{d}:vsync\n"),
        JankType::Soft => format!("{t}:{d}:soft\n"),
    };

    fs::write(p, message)?;

    Ok(())
}

fn get_refresh_rate() -> Option<u32> {
    let dumpsys_data = Command::new("dumpsys")
        .arg("SurfaceFlinger")
        .output()
        .ok()?;
    let dumpsys_data = String::from_utf8_lossy(&dumpsys_data.stdout);

    let parse_line = dumpsys_data
        .lines()
        .find(|line| line.contains("refresh-rate"))?;
    Some(
        parse_line
            .split(':')
            .nth(1)?
            .split('.')
            .next()?
            .trim()
            .parse()
            .unwrap(),
    )
}
