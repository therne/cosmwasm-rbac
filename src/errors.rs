use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RbacError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("unauthorized")]
    Unauthorized,

    #[error("duplicated role: {0} already has the role")]
    DuplicatedRole(Addr),

    #[error("no role to revoke: {0} doesn't have the role")]
    NoRoleToRevoke(Addr),
}
