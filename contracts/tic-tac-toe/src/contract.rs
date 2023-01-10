use crate::errors::ContractError;
use crate::execution::{try_accept, try_invite, try_play, try_reject};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_game, query_games};
use crate::state::GAMES_COUNT;
use cosmwasm_std::to_binary;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version};

// version info for migration info
const CONTRACT_NAME: &str = "tic-tac-toe";
const CONTRACT_VERSION: &str = "0.1.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    GAMES_COUNT.save(deps.storage, &0)?;
    Ok(Response::new().add_attribute("action", "tic-tac-toe"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Invite { coord, opponent } => try_invite(deps, info, coord, opponent),
        ExecuteMsg::Reject {
            as_host,
            opponent,
            game_id,
        } => try_reject(deps, info, as_host, opponent, game_id),
        ExecuteMsg::Accept {
            coord,
            host,
            game_id,
        } => try_accept(deps, info, coord, host, game_id),
        ExecuteMsg::Play {
            as_host,
            coord,
            opponent,
            game_id,
        } => try_play(deps, info, as_host, coord, opponent, game_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::Game { key, game_id } => to_binary(&query_game(deps, key, game_id)?),
        QueryMsg::Games { status } => to_binary(&query_games(deps, status)?),
    }
}
