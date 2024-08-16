use crate::config;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule: config::ConfigModule {
    fn emit_contract_locked_event(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        self.contract_locked_event(&entity, &contract);
    }

    fn emit_contract_unlocked_event(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        self.contract_unlocked_event(&entity, &contract);
    }

    #[event("contractLocked")]
    fn contract_locked_event(&self, #[indexed] entity: &ManagedAddress, #[indexed] contract: &ManagedAddress);

    #[event("contractUnlocked")]
    fn contract_unlocked_event(&self, #[indexed] entity: &ManagedAddress, #[indexed] contract: &ManagedAddress);
}
