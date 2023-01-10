#[cfg(not(feature = "library"))]
use cosmwasm_std::{Addr, BankMsg, DepsMut, MessageInfo, Response};

use crate::errors::ContractError;
use crate::state::{Coord, Game, Status, GAMES, GAMES_COUNT};

pub fn try_invite(
    deps: DepsMut,
    info: MessageInfo,
    coord: Coord,
    opponent: String,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }

    if opponent_address == info.sender {
        return Err(ContractError::CannotStartGame {});
    }

    let mut game_id: u64 = GAMES_COUNT.load(deps.storage)?;
    game_id = game_id + 1;
    GAMES_COUNT.save(deps.storage, &game_id)?;

    let game = Game::new(coord, info.funds);
    GAMES.save(
        deps.storage,
        (&info.sender, &opponent_address, game_id),
        &game,
    )?;

    Ok(Response::new()
        .add_attribute("method", "invite")
        .add_attribute("x", coord.x.to_string())
        .add_attribute("y", coord.y.to_string())
        .add_attribute("game_id", game_id.to_string())
        .add_attribute("opponent", opponent))
}

pub fn try_reject(
    deps: DepsMut,
    info: MessageInfo,
    as_host: bool,
    opponent: String,
    game_id: u64,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    let key: (&Addr, &Addr, u64);
    let refund_address: &Addr;

    if as_host {
        key = (&info.sender, &opponent_address, game_id);
        refund_address = &info.sender;
    } else {
        key = (&opponent_address, &info.sender, game_id);
        refund_address = &opponent_address;
    };

    let game = GAMES
        .may_load(deps.storage, key)
        .unwrap()
        .filter(|game| game.status == Status::INVITED);

    if game.is_none() {
        return Err(ContractError::GameNotFound {
            host: info.sender,
            opponent: opponent_address,
        });
    } else {
        let mut game = game.unwrap();
        game.status = Status::REJECTED;
        GAMES.save(deps.storage, key, &game)?;

        Ok(Response::new()
            .add_attribute("method", "reject")
            .add_attribute("opponent", opponent)
            .add_message(BankMsg::Send {
                to_address: refund_address.to_string(),
                amount: game.prize.clone(),
            }))
    }
}

pub fn try_accept(
    deps: DepsMut,
    info: MessageInfo,
    coord: Coord,
    host: String,
    game_id: u64,
) -> Result<Response, ContractError> {
    let host_address = deps.api.addr_validate(&host)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }

    let game = GAMES
        .may_load(deps.storage, (&host_address, &info.sender, game_id))
        .unwrap()
        .filter(|game| game.status == Status::INVITED);

    if game.is_none() {
        return Err(ContractError::InvalidGame {
            host: info.sender,
            opponent: host_address,
        });
    } else {
        let mut game = game.unwrap();
        if game.already_played_on(coord) {
            return Err(ContractError::CoordinateAlreadyPlayed { coord });
        } else if game.prize.ne(&info.funds) {
            return Err(ContractError::InvalidReceivedFunds {});
        }
        let game = game.double_prize().play(coord).finish_round();
        game.status = Status::PLAYING;

        GAMES.save(deps.storage, (&host_address, &info.sender, game_id), game)?;
    }

    Ok(Response::new()
        .add_attribute("method", "accept")
        .add_attribute("x", coord.x.to_string())
        .add_attribute("y", coord.y.to_string())
        .add_attribute("opponent", host_address))
}

pub fn try_play(
    deps: DepsMut,
    info: MessageInfo,
    as_host: bool,
    coord: Coord,
    opponent: String,
    game_id: u64,
) -> Result<Response, ContractError> {
    let opponent_address = deps.api.addr_validate(&opponent)?;
    if !coord.is_valid() {
        return Err(ContractError::InvalidCoord { coord });
    }
    let key = if as_host {
        (&info.sender, &opponent_address, game_id)
    } else {
        (&opponent_address, &info.sender, game_id)
    };

    let game = GAMES
        .may_load(deps.storage, key)
        .unwrap()
        .filter(|game| game.status == Status::PLAYING);

    if game.is_none() {
        return Err(ContractError::InvalidGame {
            host: info.sender,
            opponent: opponent_address,
        });
    } else {
        let mut game = game.unwrap();
        if game.already_played_on(coord) {
            return Err(ContractError::CoordinateAlreadyPlayed { coord });
        } else if game.already_played(as_host) {
            return Err(ContractError::TurnAlreadyPlayed {
                second_player: opponent,
            });
        }

        let game = game.play(coord);

        if game.is_current_player_winner() {
            game.status = Status::COMPLETED;
            game.winner = Some(game.player_round.unwrap());
            game.player_round = None;
        } else if game.is_full_board() {
            game.status = Status::COMPLETED;
            game.player_round = None;
        } else {
            game.finish_round();
        }

        GAMES.save(deps.storage, key, game)?;

        let res = Response::new()
            .add_attribute("method", "play")
            .add_attribute("x", coord.x.to_string())
            .add_attribute("y", coord.y.to_string())
            .add_attribute("status", game.status.to_string())
            .add_attribute("opponent", opponent.clone());

        if game.status == Status::COMPLETED {
            if game.winner.is_some() {
                return Ok(res
                    .add_attribute("winner", game.winner.unwrap().to_string())
                    .add_message(BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: game.prize.clone(),
                    }));
            } else {
                let prize = game.get_half_prize();

                return Ok(res.add_messages(vec![
                    BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: prize.clone(),
                    },
                    BankMsg::Send {
                        to_address: opponent,
                        amount: prize.clone(),
                    },
                ]));
            }
        }

        Ok(res)
    }
}
