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
//     println!("engine_hello from src/core/engines");
// }

pub fn load_flow_state(input: String) -> Result<FlowState, serde_json::Error> {
    debug!("load_flow_state");
    let flow_state = serde_json::from_str(&input);
    flow_state
}

pub fn load_config(input: String) -> Result<Config, serde_json::Error> {
    debug!("load_config");
    let config = serde_json::from_str(&input);
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
    // TODO load the config first as a separate step

    if config.is_ok() && input.is_ok() {
        debug!(
            "config & input are: {:?}/{:?} -> evaluating",
            config.is_ok(),
            input.is_ok()
        );
        let uinput = input.unwrap();
        let uid = uinput.unique_id.clone();
        let actions = evaluate(&config.unwrap(), uinput);

        // handle the error cases
        if actions.is_err() {
            return ActionState::error("error during evaluation".to_string()).to_string();
        }
        let action: ActionState = ActionState {
            unique_id: uid,
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

pub fn evaluate(
    config: &Config,
    input: FlowState,
) -> Result<HashMap<String, Vec<Vec<Actions>>>, String> {
    debug!(
        "config: {:?}, flow: {:?}",
        config.version_name, input.unique_id
    );
    //
    // ASSUMPTION -- non stateful, so need/expect to iter states
    //

    // flow for flow in  flowroutes
    //    conditions for conditions in

    // This is the actions that result from evaluating all states given
    let mut action_transitions: HashMap<String, Vec<Vec<Actions>>> = HashMap::new();

    for (r, route) in config.flow_routes.iter().enumerate() {
        debug!(">{:?} flow_route_name: {:?}", r, route.flow_route_name);
        //
        // For each flow we need to iter through the states,
        // but only evaluate those conditional routes defined
        // as an allowable next route for evaluation.
        // Thus we need a vec of the routes_to_eval.
        // We also need a hashmap of the route name > index
        //
        let route_name_index = build_route_name_index(route);
        let mut routes_to_eval = vec!["START".to_string()]; // set of next available routes

        // This is the temporary vec of result from evaluating all states in given flow
        let mut temp_action_transitions = Vec::<Vec<Actions>>::new();

        // here we are iterating over the input states
        // the output will be the same length as the input sequence with null
        // entries between state transitions
        for (s, state) in input.state_transitions.iter().enumerate() {
            debug!(
                ">{:?}>{:?} state[0]={:?} being evaluated against routes: {:?}",
                r, s, state[0].param_key, routes_to_eval
            );
            let mut action_state = Vec::<Actions>::new();
            let mut route_loop_result: bool = false;

            // iter over the available next nodes
            'routeloop: for (e, eval_route) in routes_to_eval.iter().enumerate() {
                debug!(
                    ">{:?}>{:?}>{:?} routeloop: {:?}, result: {:?}",
                    r, s, e, eval_route, route_loop_result
                );
                let index = route_name_index.get(&eval_route.to_string());

                // does the state match all the conditions for a transition
                // let result = match index {
                //     Some(&ind) => eval_condition_all(&route.flow_conditional_matches[ind], state),
                //     None => {
                //         debug!("RESULT: none");
                //         false
                //     }
                // };
                route_loop_result = match index {
                    Some(&ind) => eval_condition_all(&route.flow_conditional_matches[ind], state),
                    None => {
                        debug!("RESULT: none");
                        false
                    }
                };

                // if the state meets the conditions write the action_value
                // to the output
                if route_loop_result == true {
                    debug!("result from eval_condition_all: true -> updating routes_to_eval");
                    // add actions
                    match index {
                        Some(&ind) => {
                            debug!("add actions index: {:?}", ind);
                            for action in route.flow_conditional_matches[ind].match_actions.iter() {
                                // if action.action_value != "NULL" {
                                //     action_state.push(action.clone());
                                // };
                                action_state.push(action.clone());
                            }
                        }
                        None => {}
                    }

                    'poproutesloop: loop {
                        routes_to_eval.pop();
                        if routes_to_eval.len() == 0 {
                            debug!("<{:?}<{:?}<{:?} break from poproutesloop", r, s, e);
                            break 'poproutesloop;
                        }
                    } // end poproutesloop

                    // since the conditions were met, we need to move the
                    // next_available_matches
                    // index for route_names_index
                    if let Some(&ind) = index {
                        debug!(
                            "new route: {:?}",
                            route.flow_conditional_matches[ind].next_available_matches
                        );
                        // let mut routes_to_eval = Vec::<String>::new();
                        for fill_route in
                            &route.flow_conditional_matches[ind].next_available_matches
                        {
                            debug!(">>>> fill next matches: {:?}", fill_route);
                            routes_to_eval.push(fill_route.to_string())
                        }
                    } else {
                        {
                            debug!("\tno route_to_eval update");
                        }
                    }
                    debug!("<{:?}<{:?} break state loop", r, s);
                    break 'routeloop;
                };
            } // end of routeloop

            // if at the end of the route_loop the result is still null
            // push a null_state
            debug!("route_loop_result: {}", route_loop_result);
            if route_loop_result == false {
                action_state.push(Actions::null_state());
            };

            // don't add the action_state vector
            // unless there is a non-zero
            debug!("action_state len: {:?}", action_state.len());
            if action_state.len() >= 1 {
                temp_action_transitions.push(action_state);
            };
        } // end of stateloop

        // use all of the action_states such that the len of the ouput
        // is the same as input
        // for act in temp_action_transitions.iter() {
        //     action_transitions.push(act.clone());
        // }
        action_transitions.insert(route.flow_route_name.clone(), temp_action_transitions);

        // // take the last instruction from the single routeloop
        // match temp_action_transitions.last() {
        //     Some(_result) => action_transitions.push(_result.to_vec()),
        //     None => debug!("temp_action_transitions.last() returned"),
        // };
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
        debug!("match_cond: {:?}", cond.param_match);
        for param in states.iter() {
            debug!("\tparams: {:?}/{:?}", param.param_key, param.param_value);
            if param.param_key == cond.param_key {
                debug!("\tparam key match!");
                bool_results.push(eval_num_string_expression(&cond, &param.param_value));
            } else {
                debug!("\tparam key: {:?} != {:?}", param.param_key, cond.param_key);
            }
        }
    }
    debug!(
        "match_conditions len: {:?} > bool_results len {:?}",
        conditions.match_conditions.len(),
        bool_results.len()
    );
    if conditions.match_conditions.len() > bool_results.len() {
        debug!("\tmatch_conditions short than bool_results: adding false");
        bool_results.push(false);
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
        eval_string_expression(value, &cond.param_match)
    } else if cond.param_type == "NUM" {
        let value = value.parse::<f64>();
        match value {
            Ok(val) => {
                debug!("\tmatch value: {:?}", val);
                let _bool = eval_num_expression(val, &cond.param_match);
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

fn eval_string_expression(value: &String, expr: &String) -> bool {
    debug!("\t\teval_string_expression: {:?}", expr);
    // let key = "$string";
    let bool_val = evalexpr::eval(
        &"str::regex_matches(\"~~\", \"||\")"
            .replace("~~", &value)
            .replace("||", &expr),
    );

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

fn eval_num_expression(value: f64, expr: &String) -> bool {
    debug!("\t\teval_num_expression: {:?}", expr);

    let bool_val = evalexpr::eval(&expr.replace("NUM", &value.to_string()));
    debug!("bool={:?}", bool_val);

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
