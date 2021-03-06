//! Peripheral access API for STM32F3 microcontrollers
//! (generated using [svd2rust])
//! [svd2rust]: https://github.com/japaric/svd2rust
//!
//! You can find an overview of the API here:
//! https://docs.rs/svd2rust/0.12.1/svd2rust/#peripheral-api
//!
//! For more details see the README here:
//! https://github.com/adamgreig/stm32-rs

#![allow(non_camel_case_types)]
#![allow(private_no_mangle_statics)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]

#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]

extern crate vcell;
extern crate bare_metal;
extern crate cortex_m;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::{default_handler, exception};

#[cfg(feature = "stm32f301")]
pub mod stm32f301;

#[cfg(feature = "stm32f302")]
pub mod stm32f302;

#[cfg(feature = "stm32f303")]
pub mod stm32f303;

#[cfg(feature = "stm32f373")]
pub mod stm32f373;

#[cfg(feature = "stm32f3x4")]
pub mod stm32f3x4;

#[cfg(feature = "stm32f3x8")]
pub mod stm32f3x8;

