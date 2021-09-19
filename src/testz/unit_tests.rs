#[cfg(test)]
mod tests {
    // use super::*;
    use crate::tests::payloads::values::{
        ACTIONS, ACTIONSTATE, BADCONFIG, BADFLOWSTATE, CONDITIONMATCHES, CONFIG, FLOW, FLOWSTATE,
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
            crate::core::engine::load_flow_state(FLOWSTATE.to_string()).unwrap();
        print!("\nFlow version: {} loaded successfully", flow.unique_id);
    }

    #[test]
    fn load_config_from_string() {
        let route_flow: crate::core::structures::Config =
            crate::core::engine::load_config(CONFIG.to_string()).unwrap();
        println!("route flow: {:?}", route_flow.version_name);
    }

    #[test]
    fn output_state_from_action_state() {
        let action_state: crate::core::structures::ActionState =
            serde_json::from_str(ACTIONSTATE).expect("unable to convert");
        let output: String = action_state.to_string();
        // let output: String = crate::core::engine::output_state(action_state);
        println!("string from action state: {:?}", output);
    }

    #[test]
    fn main_equivalent() {
        let config: crate::core::structures::Config =
            crate::core::engine::load_config(CONFIG.to_string()).unwrap();
        let input: crate::core::structures::FlowState =
            crate::core::engine::load_flow_state(FLOWSTATE.to_string()).unwrap();
        //
        let actions = crate::core::engine::evaluate(&config, input);
        let action = crate::core::structures::ActionState {
            unique_id: "testid".to_string(),
            action_transitions: actions.unwrap(),
        };
        // let action = crate::core::engine::output_state(action);
        let action = action.to_string();
        println!("action state result: {}", action);

        //
    }

    ////////////////////
    // error handling //
    ////////////////////
    #[test]
    fn load_bad_flow_state_from_string() {
        // let flow: crate::core::structures::FlowState =
        //     crate::core::engine::load_flow_state(BADFLOWSTATE.to_string());
        let flow = crate::core::engine::load_flow_state(BADFLOWSTATE.to_string());

        assert!(flow.is_err() == true);
        print!("\nBAD Flow version: {} failed elegantly", flow.is_err());
    }

    #[test]
    fn load_bad_config_from_string() {
        // let route_flow: crate::core::structures::Config =
        //     crate::core::engine::load_config(BADCONFIG.to_string());
        let route_flow = crate::core::engine::load_config(BADCONFIG.to_string());
        assert!(route_flow.is_err() == true);
        println!("BAD config: {:?}", route_flow.is_err());
    }

    #[test]
    fn main_bad_config_equivalent() {
        let output = crate::core::structures::ActionState::error(
            "Configuration file corrupted. See just-flow.example_configuration for an example."
                .to_string(),
        )
        .to_string();
        let result = crate::core::engine::process(BADCONFIG.to_string(), FLOWSTATE.to_string());
        assert!(result == output);
    }

    #[test]
    fn main_bad_flow_equivalent() {
        let result = crate::core::engine::process(CONFIG.to_string(), BADFLOWSTATE.to_string());
        let output = crate::core::structures::ActionState::error(
            "Input state file corrupted. See just-flow.example_flow_state for an example."
                .to_string(),
        )
        .to_string();
        assert!(result == output);
    }
    //////////////////
    // end of tests //
    //////////////////
}
