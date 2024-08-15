multiversx_sc::imports!();

use contract_stage::config::*;
use contract_stage::*;
use multiversx_sc_scenario::imports::*;
use multiversx_sc_scenario::*;

pub const WASM_PATH: &'static str = "output/contract-stage.wasm";

#[allow(dead_code)]
pub struct ContractSetup<ObjBuilder>
where
    ObjBuilder: 'static + Copy + Fn() -> contract_stage::ContractObj<DebugApi>,
{
    pub blockchain: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub trusted_host: Address,
    pub contract: ContractObjWrapper<contract_stage::ContractObj<DebugApi>, ObjBuilder>,
}

pub fn setup_contract<ObjBuilder>(builder: ObjBuilder) -> ContractSetup<ObjBuilder>
where
    ObjBuilder: 'static + Copy + Fn() -> contract_stage::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain = BlockchainStateWrapper::new();
    let owner_address = blockchain.create_user_account(&rust_zero);
    let user_address = blockchain.create_user_account(&rust_zero);
    let trusted_host = blockchain.create_user_account(&rust_zero);
    let contract = blockchain.create_sc_account(&rust_zero, Some(&owner_address), builder, WASM_PATH);

    blockchain
        .execute_tx(&owner_address, &contract, &rust_zero, |sc| {
            sc.init(managed_address!(&trusted_host));
        })
        .assert_ok();

    ContractSetup {
        blockchain,
        owner_address,
        user_address,
        trusted_host,
        contract,
    }
}

#[test]
fn it_initializes_the_contract() {
    let mut setup = setup_contract(contract_stage::contract_obj);
    let trusted_host = setup.trusted_host.clone();

    setup
        .blockchain
        .execute_query(&setup.contract, |sc| {
            assert_eq!(managed_address!(&trusted_host), sc.trusted_host().get());
        })
        .assert_ok();
}
