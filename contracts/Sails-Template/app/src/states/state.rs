// necesary cretes
use sails_rs::{
    prelude::*,
    collections::HashMap,
};

// 1. Create the main state as a static variable.
pub static mut STATE: Option<CustomStruct> = None;

// Create a Main State
#[derive(Clone, Default)]
pub struct CustomStruct {
    pub firstfield: String,
    pub secondfield: String,
    pub thirdfield: u128,
    pub fourthfield: HashMap<ActorId, CustomInput> ,
    pub fifthfield: HashMap<ActorId, u128>,
}

// Create your own Struct
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoCustomStruct {
    pub firstfield: String,
    pub secondfield: String,
    pub thirdfield: u128,
    pub fourthfield: Vec<(ActorId, CustomInput)> ,
    pub fifthfield: Vec<(ActorId, u128)> ,
}

#[derive(Debug, Decode, Encode,  Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CustomInput {
   pub firstfield: String,
   pub secondfield: u128,
   pub thirdfield: ActorId,
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<CustomStruct> for IoCustomStruct {
    // Conversion method
    fn from(value: CustomStruct) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let CustomStruct {
            firstfield,
            secondfield,
            thirdfield,
            fourthfield,
            fifthfield,
        } = value;

        // Perform some transformation, cloning its elements
        let fourthfield = fourthfield.into_iter().collect();
        let fifthfield = fifthfield.into_iter().collect();

        // Create a new IoCustomStruct object using the destructured fields
        Self {
            firstfield,
            secondfield,
            thirdfield,
            fourthfield,
            fifthfield,
        }
    }
}

