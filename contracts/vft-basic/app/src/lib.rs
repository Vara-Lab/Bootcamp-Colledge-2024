#![no_std]
use sails_rs::{collections::HashMap, prelude::*};

pub struct State {
    name: String,
    balances: HashMap<ActorId, U256>,
}

static mut STATE: Option<State> = None;

impl State {
    pub fn get() -> &'static Self {
        unsafe { STATE.as_ref().expect("State is not initialized") }
    }

    pub fn get_mut() -> &'static mut Self {
        unsafe { STATE.as_mut().expect("State is not initialized") }
    }
}

#[derive(Default)]
pub struct Token;

#[derive(Encode, TypeInfo)]
enum Events {
    Transferred {
        from: ActorId,
        to: ActorId,
        value: U256,
    },
    Minted { to: ActorId, value: U256 },
}

#[service(events = Events)]
impl Token {
    pub fn init(name: String) {
        unsafe {
            STATE = Some(State {
                name,
                balances: HashMap::new(),
            });
        }
    }

    pub fn mint(&mut self, to: ActorId, value: U256) {
        let state = State::get_mut();
        let balance = state.balances.entry(to).or_insert(U256::zero());
        *balance += value;
        let _ = self.notify_on(Events::Minted { to, value });
    }

    pub fn transfer(&mut self, from: ActorId, to: ActorId, value: U256) {
        let state = State::get_mut();
        let from_balance = state.balances.entry(from).or_insert(U256::zero());

        if *from_balance < value {
            panic!("Insufficient balance");
        }

        *from_balance -= value;
        let to_balance = state.balances.entry(to).or_insert(U256::zero());
        *to_balance += value;

        let _ = self.notify_on(Events::Transferred { from, to, value });
    }

    pub fn name(&self) -> &'static str {
        let state = State::get();
        &state.name
    }

    pub fn balance_of(&self, account: ActorId) -> U256 {
        let state = State::get();
        *state.balances.get(&account).unwrap_or(&U256::zero())
    }
}

pub struct MyProgram;

#[program]
impl MyProgram {
    pub fn new(name: String) -> Self {
        Token::init(name);
        Self
    }

    pub fn token(&self) -> Token {
        Token::default()
    }
}