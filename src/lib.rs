#![no_std]

multiversx_sc::imports!();

pub mod config;
pub mod events;
pub mod errors;

#[multiversx_sc::contract]
pub trait ContractState: config::ConfigModule + events::EventsModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
