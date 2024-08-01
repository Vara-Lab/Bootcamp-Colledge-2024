#![no_std]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use gstd::String;
use sails_rs::gstd::program;
use services::vft;

mod services;

pub struct Program(());

#[program]
impl Program {
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        <vft::Service>::seed(name, symbol, decimals);
        Self(())
    }

    pub fn vft(&self) -> vft::Service {
        vft::Service::new()
    }
}
