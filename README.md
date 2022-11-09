# cosmwasm-rbac

Utility for implementing Role-Based Access Control (RBAC) on CosmWasm.

## Examples

### Defining Roles

The main interface is `Role`, which is basically a state containing addresses who having the role.
It supports constant initializer so feel free to define it in `cw_storage_plus` style.

For example, in your `state.rs`:

```rust
use cosmwasm_rbac::Role;

const ADMINS: Role = Role::new("admins");
const USERS: Role = Role::new("users");
```

### Granting / Revoking Roles

Use `grant` and `revoke` method to add or remove roles from an account.

```
ADMINS.grant(deps.storage, address)?;
USERS.revoke(deps.storage, address)?;
```

### Checking Roles

`has(Deps, &Addr)` method returns `true` if given address has the role.

```
if !ADMINS.has(deps, &info.sender) {
    // not an admin! raise your error here
    return Err(ContractError::Unauthorized {})
}
```

Alternatively, if your `ContractError` contains a `cosmwasm_rbac::RbacError`, you can just simply use **`check(Deps, &Addr)`** method, which returns `Result<RbacError>`:

```
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    // if you contain `RbacError`
    #[error("{0}")]
    Rbac(#[from] cosmwasm_rbac::RbacError),
}

// then you can simply check  (`check` returns `Result`)
ADMINS.check(deps, &address)?;
```

### Use Common Query/Execute Handler

`cosmwasm_rbac` provides useful common queries/executions for managing RBACs. For example, in your query message and handler, please define enums bearing `RbacQueryMsg` per your roles and call `handle_query(...)` on the handler.

```

#[cw_serde]

pub enum QueryMsg {
    Admin(cosmwasm_rbac::RbacQueryMsg),
    // ...your queries here
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        Admin(rbac_msg) => Ok(ADMINS.handle_query(deps, rbac_msg)?),
        // ...your query handler here
    }
}
```

this adds following queries: (for details, please refer to [`src/query.rs`](./src/query.rs))

```
{
  "admin": {
    "has_role": {
      "address": "<address>"
    }
  }
}
{
  "admin": {
    "all_accounts": {}
  }
}
```

unlike the query, for executions **be warned that you need to check the permissions manually** before calling `handle_execute`.
IF YOU NOT, ANYONE COULD MODIFY YOUR ROLES!

```
#[cw_serde]
pub enum ExecuteMsg {
    User(cosmwasm_rbac::RbacExecuteMsg),
    // ...your executions here
}

#[cw_serde]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        User(rbac_msg) => {
            // CHECK NEEDED: only admin can manage minters
            ADMINS.check(deps, &info.sender)?;

            Ok(USERS.handle_execute(deps, info, msg)?)
        },
        // ...your execution handler here
    }
}
```

this adds following executions: (for details, please refer to [`/src/execute.rs`](/src/execute.rs))

```
{"admin": {"grant": {"address": "<address>"}}}
{"admin": {"revoke": {"address": "<address>"}}}
{"admin": {"transfer": {"to": "<address>"}}}
```


## License: MIT
