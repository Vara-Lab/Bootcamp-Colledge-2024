// necesary cretes
use sails_rs::{
    prelude::*,
    collections::HashMap,
};

// 1. Create the state for traffic light service as a static variable.
pub static mut TRAFFIC_LIGHT_STATE: Option<TrafficLightState> = None;

// Create a struct for the state
#[derive(Clone, Default)]
pub struct TrafficLightState {
    pub current_light: String,
    pub all_users: HashMap<ActorId, String>,
}

// Create a struct that can be send to the user who reads state
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoTrafficLightState {
    pub current_light: String,
    pub all_users: Vec<(ActorId, String)>,
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<TrafficLightState> for IoTrafficLightState {

    // Conversion method
    fn from(value: TrafficLightState) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let TrafficLightState {
            current_light,
            all_users,
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let all_users = all_users
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            current_light,
            all_users,
        }
    }
}

// Create the mutability function for your state.
pub fn traffic_light_state_mut() -> &'static mut TrafficLightState {
    let state = unsafe { TRAFFIC_LIGHT_STATE.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

// Create the reference function for your state.
pub fn traffic_light_state_ref() -> &'static TrafficLightState {
    let state = unsafe { TRAFFIC_LIGHT_STATE.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}