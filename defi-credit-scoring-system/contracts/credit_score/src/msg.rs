// msg.rs - Define messages for contract interaction
use cosmwasm_std::Addr;

pub struct QueryMsg {
    pub user: Addr,
}
