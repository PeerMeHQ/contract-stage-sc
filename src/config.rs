multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type UserId = usize;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[storage_mapper("users")]
    fn users(&self) -> UserMapper;
}
