// mod structures;
use std::collections::HashMap;

use crate::core::structures::{
    ActionState, Actions, ConditionMatches, Config, FlowState, MatchCondition, RouteFlow, State,
};
// use crate::testz::payloads::values::AVALUE;

use log::{debug, error};

// pub fn engine_hello() {
//     // a function to say hello form src/core/engine.rs
//     debug!("engine_hello started");
//     println!("avalue here: {}", AVALUE);
//     println!("engine_hello from src/core/engine.rs");
// }

pub fn load_flow_state(input: String) -> Result<FlowState, serde_json::Error> {
    debug!("load_flow_state");
    // TODO needs to return ok/err
    let flow_state = serde_json::from_str(&input);
    // let flow_state = match flow_state {
    //     Ok(flow) => flow,
    //     Err(error) => {
    //         error!("error from input: {:?} with error: {:?}", &input, &error);
    //         panic!("Flow state is incorrectly formatted: {:?}", error);
    //     }
    // };
    flow_state
}

pub fn load_config(input: String) -> Result<Config, serde_json::Error> {
    debug!("load_config");
    // TODO needs to return ok/err
    let config = serde_json::from_str(&input);
    // let config = match config {
    //     Ok(flow) => flow,
    //     Err(error) => {
    //         error!("error from config: {:?} with error: {:?}", &input, &error);
    //         panic!("Config is incorrectly formatted: {:?}", error);
    //     }
    // };
    config
}

pub fn process(config: String, input: String) -> String {
    debug!("process called");
    //
    // if all is well process, otherwise return an
    // error payload
    //
    let config = crate::core::engine::load_config(config);
    let input = crate::core::engine::load_flow_state(input);

    if config.is_ok() && input.is_ok() {
        debug!(
            "config & input are: {:?}/{:?} -> evaluating",
            config.is_ok(),
            input.is_ok()
        );
        let actions = evaluate(&config.unwrap(), input.unwrap());
        if actions.is_err() {
            return ActionState::error("error during evaluation".to_string()).to_string();
        }
        let action: ActionState = ActionState {
            unique_id: "some unique id".to_string(),
            action_transitions: actions.unwrap(),
        };
        // "result".to_string();
        action.to_string()
    } else {
        error!("config or input bad. returning error action");
        if config.is_err() {
            ActionState::error(
                "Configuration file corrupted. See just-flow.example_configuration for an example."
                    .to_string(),
            )
            .to_string()
        } else if input.is_err() {
            ActionState::error(
                "Input state file corrupted. See just-flow.example_flow_state for an example."
                    .to_string(),
            )
            .to_string()
        } else {
            ActionState::error("Here is the error message".to_string()).to_string()
        }
    }
}

pub fn evaluate(config: &Config, input: FlowState) -> Result<Vec<Vec<Actions>>, String> {
    debug!(
        "config: {:?}, flow: {:?}",
        config.version_name, input.unique_id
    );
    // TODO needs to return ok/err

    //
    // ASSUMPTION -- non stateful, so need/expect to iter states
    //

    // flow for flow in  flowroutes
    //    conditions for conditions in

    // This is the actions that result from evaluating all states given
    let mut action_transitions = Vec::<Vec<Actions>>::new();

    for route in config.flow_routes.iter() {
        debug!(">flow_route_name: {:?}", route.flow_route_name);
        //
        // For each flow we need to iter through the states,
        // but only evaluate those conditional routes defined
        // as an allowable next route for evaluation.
        // Thus we need a vec of the routes_to_eval.
        // We also need a hashmap of the route name > index
        //
        let route_name_index = build_route_name_index(route);
        let mut routes_to_eval = vec!["START".to_string()];

        for state in input.state_transitions.iter() {
            debug!(
                ">>state[0] being evaluated: {:?} against routes: {:?}",
                state[0].param_name, routes_to_eval
            );
            let mut action_state = Vec::<Actions>::new();

            'routeloop: for eval_route in routes_to_eval.iter() {
                debug!(">>>routeloop: {:?}", eval_route);
                let index = route_name_index.get(&eval_route.to_string());

                let result = match index {
                    Some(&ind) => eval_condition_all(&route.flow_conditional_matches[ind], state),
                    None => {
                        debug!("RESULT: none");
                        false
                    }
                };

                if result == true {
                    debug!("result from eval_condition_all: true -> updating routes_to_eval");
                    // add actions
                    match index {
                        Some(&ind) => {
                            debug!("add actions index: {:?}", ind);
                            for action in route.flow_conditional_matches[ind].match_actions.iter() {
                                action_state.push(action.clone());
                            }
                        }
                        None => {}
                    }

                    'poproutesloop: loop {
                        routes_to_eval.pop();
                        if routes_to_eval.len() == 0 {
                            debug!("break from poproutesloop");
                            break 'poproutesloop;
                        }
                    }
                    if let Some(&ind) = index {
                        debug!(
                            "new route: {:?}",
                            route.flow_conditional_matches[ind].next_available_matches
                        );
                        // let mut routes_to_eval = Vec::<String>::new();
                        for fill_route in
                            &route.flow_conditional_matches[ind].next_available_matches
                        {
                            debug!(">>>>fill next matches: {:?}", fill_route);
                            routes_to_eval.push(fill_route.to_string())
                        }
                    } else {
                        {
                            debug!("\tno route_to_eval update");
                        }
                    }
                    debug!("break state loop");
                    break 'routeloop;
                };
            } // end of routeloop
            debug!("action_state len: {:?}", action_state.len());
            action_transitions.push(action_state);
        } // end of stateloop
        debug!("action_transitions len: {:?}", action_transitions.len());
    } // end of flowrouteloop
    Ok(action_transitions)
}

fn build_route_name_index(route: &RouteFlow) -> HashMap<String, usize> {
    debug!("route_name_index called");
    let mut route_name_index = HashMap::<String, usize>::new();
    for (i, conditions) in route.flow_conditional_matches.iter().enumerate() {
        let key = &conditions.route_condition_name;
        route_name_index.insert(key.to_string(), i);
    }
    debug!("route_name_index: {:?}", route_name_index);
    route_name_index
}

fn eval_condition_all(conditions: &ConditionMatches, states: &Vec<State>) -> bool {
    debug!(
        "eval_flow: route_condition_name: {:?}",
        conditions.route_condition_name
    );
    //
    // given a single condition, eval if the given state vector meets that condition
    //
    let mut bool_results = Vec::<bool>::new();
    for cond in conditions.match_conditions.iter() {
        debug!(
            "match_key: {:?}, match_cond: {:?}",
            cond.param_key, cond.param_match
        );
        for param in states.iter() {
            debug!("\tparams: {:?}/{:?}", param.param_name, param.param_value);
            if param.param_name == cond.param_key {
                debug!("\tparam key match!");
                bool_results.push(eval_num_string_expression(&cond, &param.param_value));
            }
        }
    }
    debug!("eval_flow bool_results: {:?}", bool_results);
    let result = eval_bool_results_all(bool_results);
    debug!("eval_flow_bool_results ouput: {:?}", result);
    result
}

fn eval_bool_results_all(vector: Vec<bool>) -> bool {
    debug!("\t\teval_bool_results for: {:?}", vector);
    vector.iter().all(|&x| x == true)
}

fn eval_num_string_expression(cond: &MatchCondition, value: &String) -> bool {
    debug!("\teval_num_string");
    //
    // evaluate a single condition against a single state param
    // these are eval differently dependent on NUM/STRING
    // the value & key are the substitutions into the expr
    //

    if cond.param_type == "STRING" {
        eval_string_expression(value, &cond.param_name, &cond.param_match)
    } else if cond.param_type == "NUM" {
        let value = value.parse::<f64>();
        match value {
            Ok(val) => {
                debug!("\tmatch value: {:?}", val);
                let _bool = eval_num_expression(val, &cond.param_name, &cond.param_match);
                debug!("\t\tbool_result: {:?}", _bool);
                _bool
            }
            Err(error) => {
                error!(
                    "NUM type given for value that cannot be parsed as number: {:?}",
                    error
                );
                false
            }
        }
    } else {
        error!(
            "type information for match condition error: {:?}",
            cond.param_type
        );
        false
    }
}

fn eval_string_expression(value: &String, key: &String, expr: &String) -> bool {
    debug!("\t\teval_string_expression: {:?}", expr);
    // let key = "$string";
    let _val = value.to_string();
    let context = evalexpr::context_map! {key => _val};
    debug!("\t\tcontext: {:?}", context);

    let context = match context {
        Ok(cont) => cont,
        Err(error) => {
            error!("failed to extract the context map: {:?}", error);
            let cont = evalexpr::context_map! {key => "NULL"};
            cont.unwrap() // TODO
        }
    };
    let bool_val = evalexpr::eval_with_context(expr, &context);
    let bool_result = match bool_val {
        Ok(val) => val.as_boolean().unwrap(), // TODO
        Err(error) => {
            error!("failed to get bool from eval, returning false: {:?}", error);
            false
        }
    };
    debug!("\t\tbool_val: {:?}", bool_result);
    bool_result
}

fn eval_num_expression(value: f64, key: &String, expr: &String) -> bool {
    debug!("\t\teval_num_expression: {:?}", expr);
    let context = evalexpr::context_map! {key => value};
    debug!("\tcontext: {:?}", context);

    let context = match context {
        Ok(cont) => cont,
        Err(error) => {
            error!("failed to extract the context map: {:?}", error);
            let cont = evalexpr::context_map! {key => 5};
            cont.unwrap() // TODO
        }
    };
    let bool_val = evalexpr::eval_with_context(expr, &context);
    let bool_result = match bool_val {
        Ok(val) => val.as_boolean().unwrap(), // TODO
        Err(error) => {
            error!("failed to get bool from eval, returning false: {:?}", error);
            false
        }
    };
    debug!("\t\tbool_val: {:?}", bool_result);
    bool_result
}
