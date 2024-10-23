// lib.rs - Exposing the contract functions
pub mod contract;

use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: String,
) -> StdResult<Response> {
    contract::execute_credit_score(deps, env, info, user)
}
