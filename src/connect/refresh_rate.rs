use std::{
    process::Command,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub fn dumper(f: &Arc<AtomicU32>) {
    loop {
        if let Some(fps) = get_refresh_rate() {
            f.store(fps, Ordering::Release);
        }

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn get_refresh_rate() -> Option<u32> {
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
