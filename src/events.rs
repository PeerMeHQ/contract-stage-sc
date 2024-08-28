use crate::config;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule: config::ConfigModule {
    fn emit_contract_staged_event(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        self.contract_staged_event(&entity, &contract);
    }

    fn emit_contract_unlocked_event(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        self.contract_unlocked_event(&entity, &contract);
    }

    #[event("contractStaged")]
    fn contract_staged_event(&self, #[indexed] entity: &ManagedAddress, #[indexed] contract: &ManagedAddress);

    #[event("contractUnlocked")]
    fn contract_unlocked_event(&self, #[indexed] entity: &ManagedAddress, #[indexed] contract: &ManagedAddress);
}
