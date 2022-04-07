//src/core/structures.rs
use serde::{Deserialize, Serialize};
use std::fmt;

///////////////////
// INPUT PAYLOAD //
///////////////////
#[derive(Serialize, Deserialize)]
pub struct FlowState {
    pub unique_id: String,
    pub state_transitions: Vec<Vec<State>>,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub param_key: String,
    pub param_value: String,
}

////////////////////
// OUTPUT PAYLOAD //
////////////////////
#[derive(Serialize, Deserialize)]
pub struct ActionState {
    pub unique_id: String,
    pub action_transitions: Vec<Vec<Actions>>,
} // end ActionState

impl fmt::Display for ActionState {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = serde_json::to_string(self);
        let json = match json {
            Ok(out) => out,
            Err(error) => "empty string".to_string(),
        };
        write!(f, "{}", json)
    }
} // end ActionState display

impl ActionState {
    #[allow(dead_code)]
    pub fn empty() -> ActionState {
        ActionState {
            unique_id: "test_empty".to_string(),
            action_transitions: vec![vec![Actions {
                action_key: "test_action_key".to_string(),
                action_value: "test_action_value".to_string(),
            }]],
        }
    }

    pub fn error(error_message: String) -> ActionState {
        ActionState {
            unique_id: error_message,
            action_transitions: vec![vec![Actions {
                action_key: "error_action_key".to_string(),
                action_value: "error_action_value".to_string(),
            }]],
        }
    }
} // end ActionState impl

#[derive(Serialize, Deserialize, Clone)]
pub struct Actions {
    pub action_key: String,
    pub action_value: String,
}

impl Actions {
    pub fn null_state() -> Actions {
        Action {
            action_key: "NULL".to_string(),
            action_value: "NULL".to_string(),
        }
    } // end null_state
} // end Actions impl

////////////////////
// CONFIG PAYLOAD //
////////////////////
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
    pub match_actions: Vec<Actions>,
    pub next_available_matches: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MatchCondition {
    pub param_key: String,
    pub param_type: String,
    pub param_match: String,
}

#[derive(Serialize, Deserialize)]
pub struct RouteName {
    pub value: String,
}
