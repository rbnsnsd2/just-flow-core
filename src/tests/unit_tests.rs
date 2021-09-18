#[cfg(test)]
mod tests {
    // use super::*;
    use crate::tests::payloads::values::{
        ACTIONS, ACTIONSTATE, CONDITIONMATCHES, CONFIG, FLOW, FLOWSTATE, MATCHACTION,
        MATCHCONDITION, ROUTEFLOW, STATE,
    };
    use log::debug;

    #[test]
    fn load_state() {
        let state: crate::core::structures::State =
            serde_json::from_str(STATE).expect("unable to convert");
        println!("param extracted: {:?}", state.param_name);
        let json: String = serde_json::to_string(&state).unwrap();
        debug!("state to string: {:?}", json);
    }

    #[test]
    fn load_flow() {
        let flow: crate::core::structures::FlowState =
            serde_json::from_str(FLOW).expect("unable to convert file to json");
        print!("\nFlow version: {} loaded successfully", flow.unique_id);
    }

    #[test]
    fn output_actions() {
        let action: crate::core::structures::Actions =
            serde_json::from_str(ACTIONS).expect("unable to convert");
        println!("action: {:?}", action.action_key);
    }

    #[test]
    fn output_action_state() {
        let action_state: crate::core::structures::ActionState =
            serde_json::from_str(ACTIONSTATE).expect("unable to convert");
        println!(
            "action_state: {:?}",
            action_state.action_transitions[0][0].action_key
        );
    }

    #[test]
    fn config_match_condition() {
        let match_condition: crate::core::structures::MatchCondition =
            serde_json::from_str(MATCHCONDITION).expect("unable to convert");
        println!("match_condition: {:?}", match_condition.param_name);
    }

    #[test]
    fn config_match_action() {
        let match_action: crate::core::structures::MatchAction =
            serde_json::from_str(MATCHACTION).expect("unable to convert");
        println!("match_condition: {:?}", match_action.action_name);
    }

    #[test]
    fn config_condition_matches() {
        let condition_matches: crate::core::structures::ConditionMatches =
            serde_json::from_str(CONDITIONMATCHES).expect("unable to convert");
        println!(
            "condition_matches: {:?}",
            condition_matches.route_condition_name
        );
    }

    #[test]
    fn config_route_flow() {
        let route_flow: crate::core::structures::RouteFlow =
            serde_json::from_str(ROUTEFLOW).expect("unable to convert");
        println!("route flow: {:?}", route_flow.flow_route_name);
    }

    //////////////////////
    // engine functions //
    //////////////////////
    #[test]
    fn load_flow_state_from_string() {
        let flow: crate::core::structures::FlowState =
            crate::core::engine::load_flow_state(FLOWSTATE.to_string());
        print!("\nFlow version: {} loaded successfully", flow.unique_id);
    }

    #[test]
    fn load_config_from_string() {
        let route_flow: crate::core::structures::Config =
            crate::core::engine::load_config(CONFIG.to_string());
        println!("route flow: {:?}", route_flow.version_name);
    }

    #[test]
    fn output_state_from_action_state() {
        let action_state: crate::core::structures::ActionState =
            serde_json::from_str(ACTIONSTATE).expect("unable to convert");
        let output: String = crate::core::engine::output_state(action_state);
        println!("string from action state: {:?}", output);
    }

    #[test]
    fn main_equivalent() {
        let config: crate::core::structures::Config =
            crate::core::engine::load_config(CONFIG.to_string());
        let input: crate::core::structures::FlowState =
            crate::core::engine::load_flow_state(FLOWSTATE.to_string());
        //
        crate::core::engine::evaluate(config, input);

        //
    }

    //////////////////
    // end of tests //
    //////////////////
}
