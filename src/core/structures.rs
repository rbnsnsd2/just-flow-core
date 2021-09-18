//src/core/structures.rs
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
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
    pub match_actions: Vec<MatchAction>,
    pub next_available_matches: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MatchCondition {
    pub param_name: String,
    pub param_key: String,
    pub param_type: String,
    pub param_match: String,
}

#[derive(Serialize, Deserialize)]
pub struct MatchAction {
    pub action_name: String,
    pub action_type: String,
    pub action_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct RouteName {
    pub value: String,
}
