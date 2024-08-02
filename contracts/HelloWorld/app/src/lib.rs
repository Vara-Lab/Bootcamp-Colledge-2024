#![no_std]
use sails_rs::prelude::*;

pub struct HelloWorld(());

#[service]
impl HelloWorld {
    pub fn greeting(&mut self) -> &'static str {
        "Hello, world!"
    }
}

pub struct MyProgram;

#[program]
impl MyProgram {
    pub fn new() -> Self {
        Self
    }
    pub fn hello_world(&self) -> HelloWorld {
        HelloWorld(())
    }
}