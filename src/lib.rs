#![deny(clippy::all)]
#![warn(clippy::nursery)]

#[cfg(not(target_os = "android"))]
#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only for aarch64 android");

mod connect;
mod error;

pub use connect::Connection;

pub(crate) const API_DIR: &str = "/dev/surfaceflinger_hook";

#[derive(Debug, Copy, Clone)]
pub struct JankLevel(u32);

#[derive(Debug)]
pub enum JankType {
    Vsync,
    Soft,
}
