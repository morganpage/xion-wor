#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
//use crate::state::{Config, State, CONFIG, STATE, STREAKS};
use crate::state::{Config, CONFIG, STREAKS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:increment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // let state = State {
    //     count: msg.count,
    //     owner: info.sender.clone(),
    // };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    //    STATE.save(deps.storage, &state)?;
    let config = Config {
        admin: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(
        Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("admin", info.sender), //  .add_attribute("count", msg.count.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ClaimStreak {} => execute::claim_streak(deps, info),
    }
}

pub mod execute {
    use crate::state::STREAKS;

    use super::*;

    pub fn claim_streak(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = info.sender.clone();
        if STREAKS.has(deps.storage, owner.clone()) {
            // update player streak
            STREAKS.update(
                deps.storage,
                owner.clone(),
                |streak| -> Result<_, ContractError> {
                    match streak {
                        Some(count) => Ok(count + 1),
                        None => Ok(1),
                    }
                },
            )?;
        } else {
            // create player streak
            STREAKS.save(deps.storage, owner.clone(), &1)?;
        }

        Ok(Response::new().add_attribute("action", "claim_streak"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStreak { address } => to_json_binary(&query::streak(deps, address)?),
    }
}

pub mod query {
    use crate::msg::GetStreakResponse;

    use super::*;

    pub fn streak(deps: Deps, address: String) -> StdResult<GetStreakResponse> {
        let addr = deps.api.addr_validate(&address)?;
        let streak = STREAKS.may_load(deps.storage, addr)?.unwrap_or(0);
        Ok(GetStreakResponse { streak })
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::GetStreakResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};

    #[test]
    fn claim_streak() {
        let mut deps = mock_dependencies();
        let info1 = mock_info("anyone1", &coins(2, "token"));
        let info2 = mock_info("anyone2", &coins(2, "token"));

        let msg_claim_streak = ExecuteMsg::ClaimStreak {};
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetStreak {
                address: "anyone1".to_string(),
            },
        )
        .unwrap();
        let value: GetStreakResponse = from_json(&res).unwrap();
        assert_eq!(0, value.streak);
        let _res = execute(deps.as_mut(), mock_env(), info1, msg_claim_streak.clone()).unwrap();
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetStreak {
                address: "anyone1".to_string(),
            },
        )
        .unwrap();
        let value: GetStreakResponse = from_json(&res).unwrap();
        assert_eq!(1, value.streak);

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetStreak {
                address: "anyone2".to_string(),
            },
        )
        .unwrap();
        let value: GetStreakResponse = from_json(&res).unwrap();
        assert_eq!(0, value.streak);
        let _res = execute(deps.as_mut(), mock_env(), info2, msg_claim_streak.clone()).unwrap();
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetStreak {
                address: "anyone2".to_string(),
            },
        )
        .unwrap();
        let value: GetStreakResponse = from_json(&res).unwrap();
        assert_eq!(1, value.streak);
    }

    #[test]

    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_json(&res).unwrap();
        // assert_eq!(0, value.count);
    }
}
