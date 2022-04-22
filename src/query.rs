use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Role;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RbacQueryMsg {
    HasRole {
        address: Addr,
    },
    AllAccounts {
        starts_after: Option<Addr>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct HasRoleResponse {
    pub has: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllAccountsResponse {
    pub accounts: Vec<Addr>,
}

impl<'a> Role<'a> {
    pub fn handle_query(&self, deps: Deps, msg: RbacQueryMsg) -> StdResult<Binary> {
        match msg {
            RbacQueryMsg::HasRole { address } => {
                let has = self.has(deps.storage, &address)?;
                Ok(to_binary(&HasRoleResponse { has })?)
            }
            RbacQueryMsg::AllAccounts {
                starts_after,
                limit,
            } => {
                let accounts = self.all_accounts(deps.storage, starts_after, limit)?;
                Ok(to_binary(&AllAccountsResponse { accounts })?)
            }
        }
    }
}
