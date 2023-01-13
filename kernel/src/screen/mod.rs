use conquer_once::raw::OnceCell;
use core::fmt::{self, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

pub mod color;
pub mod macros;
pub mod vga;

// use super::color::{Buffer, Color, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

// pub static SCREEN: OnceCell<LockedScreenBuffer> = OnceCell::uninit();

// pub struct LockedScreenBuffer {}
