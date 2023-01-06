use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cw2::ContractVersion;

use crate::contract::instantiate;
use crate::contract::query;
use crate::contract::{execute, CONTRACT_NAME, CONTRACT_VERSION};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[test]
fn test_basic() {
    let mut deps = mock_dependencies();
    let host_info = mock_info("owner", &[]);
    instantiate(
        deps.as_mut(),
        mock_env(),
        host_info.clone(),
        InstantiateMsg {},
    )
    .unwrap();

    let fetch_contract_version_query: QueryMsg = QueryMsg::GetContractVersion {};
    let contract_version: ContractVersion =
        from_binary(&query(deps.as_ref(), mock_env(), fetch_contract_version_query).unwrap())
            .unwrap();
    assert_eq!(contract_version.contract, CONTRACT_NAME);
    assert_eq!(contract_version.version, CONTRACT_VERSION);

    let fetch_counter_query: QueryMsg = QueryMsg::FetchCounter {};
    let counter: u32 =
        from_binary(&query(deps.as_ref(), mock_env(), fetch_counter_query).unwrap()).unwrap();
    assert_eq!(counter, 0);
}

#[test]
fn test_increment() {
    let mut deps = mock_dependencies();
    let info = mock_info("owner", &[]);
    instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {}).unwrap();

    let value = 15;
    let increase_by_msg: ExecuteMsg = ExecuteMsg::IncreaseBy { value };
    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, increase_by_msg).is_ok(),
        true
    );

    let fetch_counter_query: QueryMsg = QueryMsg::FetchCounter {};
    let counter: u32 =
        from_binary(&query(deps.as_ref(), mock_env(), fetch_counter_query).unwrap()).unwrap();
    assert_eq!(counter, value);
}

#[test]
fn test_reset() {
    let mut deps = mock_dependencies();
    let info = mock_info("owner", &[]);
    instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {}).unwrap();

    let reset_msg: ExecuteMsg = ExecuteMsg::Reset {};
    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, reset_msg).is_ok(),
        true
    );

    let fetch_counter_query: QueryMsg = QueryMsg::FetchCounter {};
    let counter: u32 =
        from_binary(&query(deps.as_ref(), mock_env(), fetch_counter_query).unwrap()).unwrap();
    assert_eq!(counter, 0);
}
