#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod duration;
mod frame_time;
mod instant;
mod updater;

pub use duration::*;
pub use frame_time::*;
pub use instant::*;
pub use updater::*;
