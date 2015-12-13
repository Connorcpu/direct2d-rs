#![feature(slice_patterns)]

extern crate winapi;
extern crate kernel32;
extern crate uuid;

pub use factory::Factory;

#[macro_use]
mod macros;

pub mod factory;
pub mod error;
pub mod math;
pub mod geometry;
pub mod stroke_style;

mod comptr;
mod load_dll;
mod helpers;
