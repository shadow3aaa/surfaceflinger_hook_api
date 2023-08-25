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
#![deny(clippy::all, clippy::pedantic)]
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
pub struct JankLevel(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JankType {
    Vsync,
    Soft,
}
