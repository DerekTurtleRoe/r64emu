#![feature(box_syntax)]

#[macro_use]
extern crate enum_map;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate static_assertions;

mod memint;
pub mod bus;
pub mod regs;
