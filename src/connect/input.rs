use std::{
    fs::File, io::prelude::*, process::Command, sync::mpsc::Receiver, thread, time::Duration,
};

use crate::{error::Result, JankType};

pub fn updater(rx: &Receiver<(u32, JankType)>, p: &mut File) {
    let display_fps = get_refresh_rate().unwrap_or_default();
    let mut status = (display_fps, display_fps, JankType::Vsync);

    loop {
        let mut temp_status = status;

        temp_status.1 = get_refresh_rate().unwrap_or_default();

        if let Ok((t, j)) = rx.try_recv() {
            temp_status.0 = t;
            temp_status.2 = j;
        }

        if status != temp_status {
            status = temp_status;

            let (t, d, j) = status;
            let _ = write_input(p, t, d, j);
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn write_input(p: &mut File, t: u32, d: u32, j: JankType) -> Result<()> {
    match j {
        JankType::Vsync => writeln!(p, "{t}:{d}:vsync")?,
        JankType::Soft => writeln!(p, "{t}:{d}:soft")?,
    }
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
