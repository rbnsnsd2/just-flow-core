//src/core/structures.rs
use serde::{Deserialize, Serialize};
use std::fmt;

// INPUT PAYLOAD
#[derive(Serialize, Deserialize)]
pub struct FlowState {
    pub unique_id: String,
    pub state_transitions: Vec<Vec<State>>,
}

// #[derive(Serialize, Deserialize)]
// pub struct State {
//     pub state_param: Vec<Param>,
// }

#[derive(Serialize, Deserialize)]
pub struct State {
    pub param_name: String,
    pub param_value: String,
}

// OUTPUT PAYLOAD
#[derive(Serialize, Deserialize)]
pub struct ActionState {
    pub unique_id: String,
    pub action_transitions: Vec<Vec<Actions>>,
}

impl fmt::Display for ActionState {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     write!(f, "{:?}, {:?}", self.unique_id, self.action_transitions)
    // }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string(self);
        let json = match json {
            Ok(out) => out,
            Err(error) => {
                error!("error formatting actionstate: {:?}", error);
                "empty string".to_string()
            }
        };
        write!(f, "{:?}", json)
    }
}

pub fn output_state(output: ActionState) -> String {
    debug!("output_state");
    let json = serde_json::to_string(&output);
    let json = match json {
        Ok(out) => out,
        Err(error) => {
            error!(
                "error from output: {:?} with error: {:?}",
                &output.unique_id, error
            );
            panic!("Actonstate output conversion to json failed: {:?}", error);
        }
    };
    debug!("output json String: {:?}", json);
    json
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Actions {
    pub action_key: String,
    pub action_value: String,
}

// CONFIG PAYLOAD
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version_name: String,
    pub stateful: bool,
    pub flow_routes: Vec<RouteFlow>,
}

#[derive(Serialize, Deserialize)]
pub struct RouteFlow {
    pub flow_route_name: String,
    pub flow_conditional_matches: Vec<ConditionMatches>,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionMatches {
    pub route_condition_name: String,
    pub match_type: String,
    pub match_condition_type: String,
    pub match_conditions: Vec<MatchCondition>,
    // pub match_actions: Vec<MatchAction>,
    pub match_actions: Vec<Actions>,
    pub next_available_matches: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MatchCondition {
    pub param_name: String,
    pub param_key: String,
    pub param_type: String,
    pub param_match: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct MatchAction {
//     pub action_name: String,
//     pub action_type: String,
//     pub action_value: String,
// }

#[derive(Serialize, Deserialize)]
pub struct RouteName {
    pub value: String,
}
