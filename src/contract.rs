use crate::config;
use crate::events;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ContractModule: config::ConfigModule + events::EventsModule {
    #[endpoint(stageContract)]
    fn stage_contract_endpoint(&self, entity: ManagedAddress, contract: ManagedAddress, code: ManagedBuffer, code_metadata: CodeMetadata, args: MultiValueEncoded<ManagedBuffer>) {
        let caller = self.blockchain().get_caller();
        let user = self.users().get_or_create_user(&caller);

        require!(self.entities().contains(&entity), "entity is not registered");
        require!(self.developers(&entity).contains(&user), "caller is not developer");
        require!(!self.is_contract_locked(&entity, &contract), "contract is locked");

        let args_buffer = args.to_arg_buffer();
        let gas = self.blockchain().get_gas_left();
        let value = BigUint::zero();

        if contract.is_zero() {
            let (new_contract, _) = self.send_raw().deploy_contract(gas, &value, &code, code_metadata, &args_buffer);
            self.lock_contract(&entity, &new_contract);
        } else {
            require!(self.contracts(&entity).contains(&contract), "contract is not registered");

            self.send_raw().upgrade_contract(&contract, gas, &value, &code, code_metadata, &args_buffer);
            self.lock_contract(&entity, &contract);
        }

        self.emit_contract_locked_event(&entity, &contract);
    }

    #[endpoint(unlockStage)]
    fn unlock_contract_stage_endpoint(&self, contract: ManagedAddress) {
        let entity = self.blockchain().get_caller();

        require!(self.entities().contains(&entity), "caller is not entity");
        require!(self.contracts(&entity).contains(&contract), "contract is not registered");
        require!(self.is_contract_locked(&entity, &contract), "stage is unlocked already");

        self.clear_stage(&entity, &contract);

        self.emit_contract_unlocked_event(&entity, &contract);
    }

    fn lock_contract(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        require!(!self.is_contract_locked(&entity, &contract), "contract is locked already");
        require!(self.blockchain().is_smart_contract(&contract), "address must be contract");
        require!(&self.blockchain().get_sc_address() != contract, "address must not be self");

        self.contract_locks(&entity, &contract).set(true);
    }

    fn clear_stage(&self, entity: &ManagedAddress, contract: &ManagedAddress) {
        self.contract_locks(&entity, &contract).clear();
    }

    fn is_contract_locked(&self, entity: &ManagedAddress, contract: &ManagedAddress) -> bool {
        !self.contract_locks(&entity, &contract).is_empty()
    }
}
