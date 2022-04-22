# cosmwasm-rbac

Utility for implementing Role-Based Access Control (RBAC) on CosmWasm.

## Examples

### Defining Roles

The main interface is `Role`, which is basically a state containing addresses who having the role. It supports constant initializer so feel free to define it in `cw_storage_plus` style.

For example, in your `state.rs`:

```rust
use cosmwasm_rbac::Role;

const ADMINS: Role = Role::new("admins");
const MINTERS: Role = Role::new("minters");
```

### Granting / Revoking Roles

Use `grant` and `revoke` method to add or remove roles from an account.

```rust
ADMINS.grant(deps.storage, address)?;
MINTERS.revoke(deps.storage, address)?;
```

### Checking Roles

`has(Deps, &Addr)` method returns `true` if given address has the role.

```rust
if !ADMINS.has(deps, &info.sender) {
    // not an admin! raise your error here
    return Err(ContractError::Unauthorized {})
}
```

Alternatively, if your `ContractError` contains a `cosmwasm_rbac::RbacError`, you can just simply use **`check(Deps, &Addr)`** method, which returns `Result<RbacError>`:

```rust
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

```rust

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
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

```js
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

unlike the query, for executions **be warn that you need to check the permissions manually** before calling `handle_execute`. IF YOU NOT, ANYONE COULD MODIFY YOUR ROLES!

```rust
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Minter(cosmwasm_rbac::RbacExecuteMsg),
    // ...your executions here
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        Minter(rbac_msg) => {
            // CHECK NEEDED: only admin can manage minters
            ADMINS.check(deps, &info.sender)?;

            Ok(MINTERS.handle_execute(deps, info, msg)?)
        },
        // ...your execution handler here
    }
}
```

this adds following executions: (for details, please refer to [`src/execute.rs`](./src/excecute.rs))

```js
{"admin": {"grant": {"address": "<address>"}}}
{"admin": {"revoke": {"address": "<address>"}}}
{"admin": {"transfer": {"to": "<address>"}}}
```

### Misc: Checking Contract Admin (Native)

apart from contract-level RBACs, there are [a notion of 'admin' in CosmWasm native level](https://github.com/CosmWasm/wasmd/blob/main/proto/cosmwasm/wasm/v1/types.proto#L69-L90). `cosmwasm_rbac` provides an utility function to query the native contract admin.

```rust
use cosmwasm_rbac::{is_contract_admin, check_contract_admin};

// manual check
if !is_contract_admin(&deps, env, &info.sender) {
  return Err(Unauthorized{})
}

// or use guards
check_contract_admin(&deps, env, &info.sender)?;
```

## License: MIT
