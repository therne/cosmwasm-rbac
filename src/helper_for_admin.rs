use cosmwasm_std::{CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Back-ported code from cosmwasm_std:v1.0.0
enum ContractInfoQuery {
    /// returns a ContractInfoResponse with metadata on the contract from the runtime
    ContractInfo { contract_addr: String },
}

impl CustomQuery for ContractInfoQuery {}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
struct ContractInfoResponse {
    pub code_id: u64,
    /// address that instantiated this contract
    pub creator: String,
    /// admin who can run migrations (if any)
    pub admin: Option<String>,
    /// if set, the contract is pinned to the cache, and thus uses less gas when called
    pub pinned: bool,
    /// set if this contract has bound an IBC port
    pub ibc_port: Option<String>,
}
//
// /// Returns `true` if contract admin.
// pub fn is_contract_admin(deps: &Deps, env: Env, address: &Addr) -> StdResult<bool> {
//     let request = ContractInfoQuery::ContractInfo {
//         contract_addr: env.contract.address.to_string(),
//     };
//     let resp: ContractInfoResponse = deps.querier.custom_query(&request.into())?;
//     let admin = resp.admin.unwrap_or_else(|| String::from(""));
//
//     Ok(admin == address.to_string())
// }

// /// Ensures that only contract admin (native) can access.
// pub fn check_contract_admin(deps: &Deps, env: Env, address: &Addr) -> Result<(), RbacError> {
//     if is_contract_admin(deps, env, address)? {
//         return Err(RbacError::Unauthorized);
//     }
//     Ok(())
// }
