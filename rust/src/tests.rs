use crate::contract::{execute, sudo};
use crate::msg::{ChainTypeInfo, ExecuteMsg, InstantiateMsg};
use crate::query::{fetch_data, fetch_deploy_state};
use crate::state::FORWARDER_CONTRACT_MAPPING;
use crate::{contract::instantiate, msg::SudoMsg};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut,
};
use cosmwasm_std::{Binary, Coin, CosmosMsg, OwnedDeps, Uint128};
use router_wasm_bindings::{RouterMsg, RouterQuery};
use std::marker::PhantomData;

const INIT_ADDRESS: &str = "router1apapk9zfz3rp4x87fsm6h0s3zd0wlmkz0fx8tx";

fn get_mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier, RouterQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}

fn do_instantiate(mut deps: DepsMut<RouterQuery>) {
    let instantiate_msg = InstantiateMsg {
        owner: INIT_ADDRESS.to_string(),
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    let env = mock_env();
    let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn test_basic() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
}

#[test]
fn test_sudo_function() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
    let env = mock_env();
    let msg: ExecuteMsg = ExecuteMsg::SetChainTypes {
        chain_type_info: vec![ChainTypeInfo {
            chain_id: "43113".into(),
            chain_type: 1,
        }],
    };
    let info = mock_info(INIT_ADDRESS, &[]);
    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let test_string: String = String::from("80001");
    let encoded_string: String = base64::encode(test_string.clone());
    let msg: SudoMsg = SudoMsg::HandleIReceive {
        request_sender: "97MRmF0DyXm9Sfa9szch6J9ie6U".to_string(),
        src_chain_id: String::from("80001"),
        request_identifier: 2,
        payload: Binary::from_base64(&encoded_string).unwrap(),
    };

    let response = sudo(deps.as_mut(), env, msg).unwrap();
    assert_eq!(response.messages.len(), 1);

    let data: String = fetch_data(deps.as_ref()).unwrap();
    assert_eq!(data, String::from("10008"));

    let message = response.messages.get(0).unwrap();
    let router_msg = message.msg.clone();
    match router_msg {
        CosmosMsg::Custom(msg) => match msg {
            RouterMsg::CrosschainCall {
                version,
                route_amount,
                route_recipient,
                dest_chain_id,
                request_metadata,
                request_packet,
            } => {
                assert_eq!(route_amount, Uint128::zero());
                assert_eq!(hex::encode(route_recipient), "");

                assert_eq!(dest_chain_id, "80001");
                assert_eq!(version, 1);
                assert_eq!(hex::encode(request_metadata), "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000493e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000493e0000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000");
                assert_eq!(hex::encode(request_packet), "000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000531303030380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014eedb3ab68d567a6cd6d19fa819fe77b9f8ed1538000000000000000000000000");
            }
        },
        _ => {}
    }
}

#[test]
fn test_sudo_ack_function() {
    let mut deps = get_mock_dependencies();
    do_instantiate(deps.as_mut());
    let env = mock_env();
    let msg: ExecuteMsg = ExecuteMsg::SetChainTypes {
        chain_type_info: vec![
            ChainTypeInfo {
                chain_id: "43113".into(),
                chain_type: 1,
            },
            ChainTypeInfo {
                chain_id: "80001".into(),
                chain_type: 1,
            },
            ChainTypeInfo {
                chain_id: "router_9000-1".into(),
                chain_type: 2,
            },
        ],
    };
    let code_hash: String =
        String::from("0xace738c68088218d015fbdce138f062893d86818ac98932f7ce2907c5976fbde");
    let salt: String =
        String::from("0x9d52787d04a49f2f9df398db0dedd78c9e543c8919f0c0024d04cd0ee8a87062");
    let chain_id: String = String::from("80001");

    let info = mock_info(INIT_ADDRESS, &[]);
    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    FORWARDER_CONTRACT_MAPPING
        .save(
            &mut deps.storage,
            &5.to_string(),
            &"router_forwarder".to_string(),
        )
        .unwrap();
    let msg: ExecuteMsg = ExecuteMsg::DeployContract {
        code: String::from("0x23"),
        salt: salt.clone(),
        constructor_args: vec![String::from("0x23")],
        chain_ids: vec![String::from("80001")],
        gas_limits: vec![3000000],
        gas_prices: vec![40_000_000_000],
    };
    let response = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(response.messages.len(), 1);

    let hex_str: &str = "0000000000000000000000000000000000000000000000000000000000013881ace738c68088218d015fbdce138f062893d86818ac98932f7ce2907c5976fbde9d52787d04a49f2f9df398db0dedd78c9e543c8919f0c0024d04cd0ee8a870620000000000000000000000000f0690a7681292b58ef2d5c3a3c9a6eeb5bf2c38";
    let binary_data: Binary = Binary(hex::decode(hex_str).unwrap());
    let msg: SudoMsg = SudoMsg::HandleIAck {
        request_identifier: 5,
        exec_flag: true,
        exec_data: binary_data,
        refund_amount: Coin {
            denom: "route".to_string(),
            amount: Uint128::zero(),
        },
    };

    let response = sudo(deps.as_mut(), env.clone(), msg).unwrap();
    assert_eq!(response.messages.len(), 1);

    let data = fetch_deploy_state(deps.as_ref(), &code_hash, &salt, &chain_id).unwrap();
    let addr: String = String::from("0x0f0690a7681292b58ef2d5c3a3c9a6eeb5bf2c38");
    assert_eq!(data.0, true);
    assert_eq!(data.1, addr);
    assert_eq!(data.2, INIT_ADDRESS);
}
