use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::COUNTER;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_binary, StdError};
use cw2::{get_contract_version, set_contract_version};

// version info for migration info
pub const CONTRACT_NAME: &str = "counter";
pub const CONTRACT_VERSION: &str = "0.1.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("action", "counter_contract_init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::IncreaseBy { value } => try_increase_counter(deps, env, info, value),
        ExecuteMsg::Reset {} => try_reset_counter(deps, env, info),
    }
}

fn try_increase_counter(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    value: u32,
) -> StdResult<Response> {
    let info_str: String = format!("updating counter value by {:?}", value);
    deps.api.debug(&info_str);

    let current_counter: u32 = COUNTER.load(deps.storage).unwrap();
    COUNTER.save(deps.storage, &(current_counter + value))?;
    let response = Response::new().add_attribute("value", value.to_string());
    Ok(response)
}

fn try_reset_counter(deps: DepsMut, _env: Env, _info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    let response = Response::new().add_attribute("counter_reset", "0");
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= CONTRACT_VERSION.to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchCounter {} => to_binary(&query_counter(deps)?),
    }
}

fn query_counter(deps: Deps) -> StdResult<u32> {
    return Ok(COUNTER.load(deps.storage)?);
}
