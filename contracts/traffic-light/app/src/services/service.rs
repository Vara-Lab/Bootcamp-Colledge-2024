use sails_rs::{
    gstd::{
        service,
        msg
    },
    prelude::*
};

// crate to sleep contract
use gstd::exec::sleep_for;

// import the state
use crate::states::state::{
    TRAFFIC_LIGHT_STATE,
    traffic_light_state_mut,
    IoTrafficLightState
};

// Traffic light service struct to build the service 
#[derive(Default)]
pub struct TrafficLightService;

// Trffic light service
#[service]
impl TrafficLightService {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Remote call "green" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub async fn green(&mut self) -> TrafficLightEvent {
        // Get state as mut
        let traffic_light_state = traffic_light_state_mut();

        let current_light = "Green".to_string();

        // Changing state
        traffic_light_state.current_light = current_light.clone();
        traffic_light_state.all_users.insert(msg::source().into(), current_light);

        // asynchronous "call"
        sleep_for(10).await;

        // returning the response
        TrafficLightEvent::Green
    }

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub async fn yellow(&mut self) -> TrafficLightEvent {
        // Get state as mut
        let traffic_light_state = traffic_light_state_mut();

        let current_light = "Yellow".to_string();

        // Changing state
        traffic_light_state.current_light = current_light.clone();
        traffic_light_state.all_users.insert(msg::source().into(), current_light);

        // asynchronous "call"
        sleep_for(20).await;

        // returning the response
        TrafficLightEvent::Yellow
    }

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub fn red(&mut self) -> TrafficLightEvent {
        // Get state as mut
        let traffic_light_state = traffic_light_state_mut();

        let current_light = "Red".to_string();

        // Changing state
        traffic_light_state.current_light =current_light.clone();
        traffic_light_state.all_users.insert(msg::source().into(), current_light);

        // returning the response
        TrafficLightEvent::Red
    }

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    pub fn traffic_light_state(&self) -> IoTrafficLightState {
        // Taking the state to be sent as a response
        let traffic_light_state = unsafe { 
            TRAFFIC_LIGHT_STATE
                .take()
                .expect("Unexpected error in taking state") 
        };

        // returning the state transforming it into the correct struct (.into()) for the user
        // This will transform the state struct (CustomStruct) to the struct that will be 
        // sent as a response (IOCustomStruct)
        traffic_light_state.into()
    }
}

// struct to use as a response to the user
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]

pub enum TrafficLightEvent {
    Green,
    Yellow,
    Red
}