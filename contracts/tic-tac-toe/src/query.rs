#[cfg(not(feature = "library"))]
use crate::data::GameResponse;
use crate::msg::QueryKey;
use crate::state::{Status, GAMES};
use cosmwasm_std::{Deps, Order, StdResult};

pub fn query_game(deps: Deps, key: QueryKey, game_id: u64) -> StdResult<Vec<GameResponse>> {
    let res: Vec<GameResponse>;

    let host_address = deps.api.addr_validate(&key.host)?;
    let opponent_address = deps.api.addr_validate(&key.opponent)?;

    let game_option = GAMES
        .may_load(deps.storage, (&host_address, &opponent_address, game_id))
        .unwrap();

    match game_option {
        Some(_game) => {
            res = vec![GameResponse {
                game: _game,
                host: host_address,
                opponent: opponent_address,
            }]
        }
        None => res = vec![],
    }

    Ok(res)
}

pub fn query_games(deps: Deps, status: Option<Status>) -> StdResult<Vec<GameResponse>> {
    let mut res: Vec<GameResponse> = GAMES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|f| {
            let (addresses, game) = f.unwrap();

            GameResponse {
                game: game,
                host: addresses.0,
                opponent: addresses.1,
            }
        })
        .collect();

    match status {
        Some(status) => {
            res = res
                .into_iter()
                .filter(|res| res.game.status == status)
                .collect()
        }
        None => {}
    }
    Ok(res)
}
