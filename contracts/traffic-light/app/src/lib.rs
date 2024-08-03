#![no_std]

// necesary crates
use sails_rs::{
    prelude::*,
    gstd::{
        program,
        route
    }
};

// import our modules 
pub mod states;
pub mod services;

// import necesary data (state and CustomStruct state)
use crate::states::state::{
    TRAFFIC_LIGHT_STATE,
    TrafficLightState
};


// Import service to be used for the program
use services::service::TrafficLightService;

// Traffic light program struct to build the program (there can only be one per contract)
#[derive(Default)]
pub struct TrafficLightProgram;

// Traffic light program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl TrafficLightProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        unsafe {
            TRAFFIC_LIGHT_STATE = Some(
                TrafficLightState::default()
            );
        };

        Self
    }

    // Method working with "&self", havind no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case TrafficLightSvc).
    #[route("TrafficLight")]
    pub fn traffic_light_svc(&self) -> TrafficLightService {
        TrafficLightService::new()
    }
}