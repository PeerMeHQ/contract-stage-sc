
multiversx_sc::imports!();

pub type UserId = usize;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[storage_mapper("users")]
    fn users(&self) -> UserMapper;

    #[storage_mapper("entities")]
    fn entities(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("devs")]
    fn developers(&self, entity: &ManagedAddress) -> UnorderedSetMapper<UserId>;

    #[storage_mapper("developers")]
    fn contracts(&self, entity: &ManagedAddress) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getContractLock)]
    #[storage_mapper("contract_locks")]
    fn contract_locks(&self, entity: &ManagedAddress, contract: &ManagedAddress) -> SingleValueMapper<bool>;
}
