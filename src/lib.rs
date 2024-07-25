#![warn(clippy::all, rust_2018_idioms)]
#![feature(iter_next_chunk, array_chunks, array_try_from_fn)]

mod app;
mod names;
pub use app::NamesApp;
