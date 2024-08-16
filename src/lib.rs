#![no_std]

multiversx_sc::imports!();

pub mod config;
pub mod events;
pub mod errors;
pub mod contract;

#[multiversx_sc::contract]
pub trait ContractState: config::ConfigModule + events::EventsModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(register)]
    fn register_endpoint(&self, developers: MultiValueEncoded<ManagedAddress>) {
        let entity = self.blockchain().get_caller();
        require!(self.entities().contains(&entity), "entity is already registered");
        require!(self.blockchain().is_smart_contract(&entity), "entity must be contract");
        require!(developers.len() > 0, "developers must be provided");

        for developer in developers.into_iter() {
            let user_dev = self.users().get_or_create_user(&developer);

            self.developers(&entity).insert(user_dev);
        }

        self.entities().insert(entity);
    }

    #[endpoint(addDeveloper)]
    fn add_developer_endpoint(&self, developer: ManagedAddress) {
        let entity = self.blockchain().get_caller();
        require!(self.entities().contains(&entity), "entity is not registered");

        let user_dev = self.users().get_or_create_user(&developer);

        self.developers(&entity).insert(user_dev);
    }

    #[endpoint(removeDeveloper)]
    fn remove_developer_endpoint(&self, developer: ManagedAddress) {
        let entity = self.blockchain().get_caller();
        require!(self.entities().contains(&entity), "entity is not registered");

        let user_dev = self.users().get_user_id(&developer);
        require!(user_dev != 0, "developer is not registered");

        self.developers(&entity).swap_remove(&user_dev);
    }
}
