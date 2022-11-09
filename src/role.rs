use cosmwasm_std::{Addr, Deps, Order, StdResult, Storage};
use cw_storage_plus::{Bound, Map};

use crate::RbacError;

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub trait RoleCheck {
    fn has(&self, deps: Deps, account: Addr) -> StdResult<bool>;
}

pub struct Role<'a> {
    pub name: &'a str,
    pub bearer: Map<'a, &'a Addr, bool>,
}

impl<'a> Role<'a> {
    pub const fn new(name: &'a str) -> Self {
        Role {
            name,
            bearer: Map::new(name),
        }
    }
}

impl<'a> Role<'a> {
    /// Returns `true` if given account has the role.
    pub fn has(&self, store: &dyn Storage, account: &Addr) -> StdResult<bool> {
        Ok(self.bearer.has(store, account))
    }

    /// Tests that given account has the role. Returns [RbacError::Unauthorized] if not.
    /// Difference with [has] method is that [check] returns [Result] with [RbacError] while [has] returns just a `bool`.
    pub fn check(&self, store: &dyn Storage, account: &Addr) -> Result<(), RbacError> {
        if self.has(store, account)? {
            Ok(())
        } else {
            Err(RbacError::Unauthorized)
        }
    }

    /// Adds an role to the account.
    pub fn grant(&self, store: &mut dyn Storage, account: Addr) -> Result<(), RbacError> {
        if self.has(store, &account)? {
            return Err(RbacError::DuplicatedRole(account));
        }
        self.bearer.save(store, &account, &true)?;
        Ok(())
    }

    /// Removes an role from the account.
    pub fn revoke(&self, store: &mut dyn Storage, account: Addr) -> Result<(), RbacError> {
        if !self.has(store, &account)? {
            return Err(RbacError::NoRoleToRevoke(account));
        }
        self.bearer.remove(store, &account);
        Ok(())
    }

    /// Returns a list of accounts having the role.
    pub fn all_accounts(
        &self,
        store: &dyn Storage,
        start_after: Option<Addr>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Addr>> {
        let start = start_after.map(|t| Bound::exclusive(t.to_string()));
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

        self.bearer
            .keys(store, start, None, Order::Ascending)
            .take(limit)
            .map(|item |Ok(Addr::unchecked(Addr::from(item.unwrap().into()))))
            // .map(|item| Ok(Addr::unchecked(String::from_utf8(item)?)))
            .collect::<StdResult<_>>()
    }
}
