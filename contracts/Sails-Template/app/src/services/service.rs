
use gstd::exec::sleep_for;
use sails_rs::{
    gstd::{
        service,
        msg
    },
    prelude::*
};

use crate::states::state::{
    STATE,
    CustomStruct,
    IoCustomStruct,
    CustomInput
};


// Create your own Events
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Events {
    // Add Events(Example)
    FirstEvent,          // Example an event with a simple input
    SecondEvent(String), // Example an event with a u128 input
    ThirdEvent(u128),    // Example an event with String input
    FourtEvent {
        first_field: ActorId,
        second_field: Vec<ActorId>, // Example an event with a custom input
    },
}

// Create your own Errors
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
    // Add errors(Example)
    FirstError,
    SecondError,
    ThirdErrors,
    FourtErrors,
}



#[derive(Default)]
pub struct Service;


#[service]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    pub async fn firstmethod(&mut self) -> Result<Events, Errors> {

        let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };

        // Update your state with a String input
        state.firstfield = "Hello".to_string();

         // asynchronous "call"
         sleep_for(10).await;

        Ok(Events::FirstEvent)
    }

    pub async fn secondmethod(&mut self, input: String) -> Result<Events, Errors> {

        let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };
        
        // Update your state with a String input
        state.secondfield = input.clone();

         // asynchronous "call"
         sleep_for(10).await;

        Ok(Events::SecondEvent(input))
    }

    pub async fn thirdmethod(&mut self, input: u128) -> Result<Events, Errors> {

        let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };

        // Update your state with a u128 input
        state.thirdfield = input;

         // asynchronous "call"
         sleep_for(10).await;

        Ok(Events::ThirdEvent(input))
    }

    pub async fn fourthmethod(&mut self, input: CustomInput) -> Result<Events, Errors> {

        let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };

        // Update your state.
        state.fourthfield 
            .entry(msg::source())
            .or_insert(CustomInput {
                firstfield: input.firstfield,
                secondfield: input.secondfield,
                thirdfield: input.thirdfield,
            });

             // asynchronous "call"
        sleep_for(10).await;

        Ok(Events::SecondEvent("Event".to_string()))
    }

    pub async fn fifthmethod(
        &mut self,
        _first_field: u128,
        _second_field: Vec<ActorId>,
    ) -> Result<Events, Errors> {


        let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("The contract is not initialized") };


        // Update your state.
        state.fifthfield
            .entry(msg::source())
            .and_modify(|number| *number = number.saturating_add(1))
            .or_insert(1);

         // asynchronous "call"
         sleep_for(10).await;

        Ok(Events::SecondEvent("Event".to_string()))
    }

 
    pub fn state(&self) -> IoCustomStruct {
       
        let state = unsafe { 
            STATE
                .take()
                .expect("Unexpected error in taking state") 
        };

        state.into()
    }
}


