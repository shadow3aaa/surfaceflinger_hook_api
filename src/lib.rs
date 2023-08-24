#![deny(clippy::all)]
#![warn(clippy::nursery)]

#[cfg(not(target_os = "android"))]
#[cfg(not(target_arch = "aarch64"))]
compile_error!("Only for aarch64 android");

mod connect;
mod error;

pub use connect::Connection;
pub use error::Error;

pub const API_DIR: &str = "/dev/surfaceflinger_hook";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JankLevel(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JankType {
    Vsync,
    Soft,
}
