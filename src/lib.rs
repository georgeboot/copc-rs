//! Library for reading and writing Cloud Optimized Point Cloud ([COPC](https://copc.io/)) data.

const MIN_NODE_SIZE_DEFAULT: i32 = 256;
const MAX_NODE_SIZE_DEFAULT: i32 = 16384;
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod compressor;
pub mod copc;
mod decompressor;
mod error;
mod hierarchy;
mod reader;
mod writer;

pub use copc::{CopcInfo, Entry, HierarchyPage, VoxelKey};
pub use error::*;
pub use hierarchy::Hierarchy;
pub use las::{Bounds, Vector, Vlr};
pub use reader::*;
pub use writer::*;
