use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Coord, Status};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Invite {
        coord: Coord,
        opponent: String,
    },
    Reject {
        as_host: bool,
        opponent: String,
        game_id: u64,
    },
    Accept {
        coord: Coord,
        host: String,
        game_id: u64,
    },
    Play {
        as_host: bool,
        coord: Coord,
        opponent: String,
        game_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContractVersion {},
    Game { key: QueryKey, game_id: u64 },
    Games { status: Option<Status> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryKey {
    pub host: String,
    pub opponent: String,
}
