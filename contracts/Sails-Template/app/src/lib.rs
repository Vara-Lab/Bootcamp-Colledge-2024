#![no_std]

use sails_rs::{
    prelude::*,
    gstd::{
        program,
        route
    }
};

pub mod states;
pub mod services;


use crate::states::state::{
    STATE,
    CustomStruct
};


use services::service::Service;


#[derive(Default)]
pub struct Program;

#[program]
impl Program {
   
    pub fn new() -> Self {
        unsafe {
            STATE = Some(
                CustomStruct::default()
            );
        };

        Self
    }

    
    #[route("Template")]
    pub fn template_svc(&self) -> Service {
        Service::new()
    }
}