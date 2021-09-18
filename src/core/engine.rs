// mod structures;
use crate::core::structures::{
    ActionState, ConditionMatches, Config, FlowState, MatchCondition, State,
};
use crate::tests::payloads::values::AVALUE;

use log::{debug, error};

pub fn engine_hello() {
    // a function to say hello form src/core/engine.rs
    debug!("engine_hello started");
    println!("avalue here: {}", AVALUE);
    println!("engine_hello from src/core/engine.rs");
}

pub fn load_flow_state(input: String) -> FlowState {
    debug!("load_flow_state");
    let flow_state = serde_json::from_str(&input);
    let flow_state = match flow_state {
        Ok(flow) => flow,
        Err(error) => {
            error!("error from input: {:?} with error: {:?}", &input, &error);
            panic!("Flow state is incorrectly formatted: {:?}", error);
        }
    };
    flow_state
}

pub fn load_config(input: String) -> Config {
    debug!("load_config");
    let config = serde_json::from_str(&input);
    let config = match config {
        Ok(flow) => flow,
        Err(error) => {
            error!("error from config: {:?} with error: {:?}", &input, &error);
            panic!("Config is incorrectly formatted: {:?}", error);
        }
    };
    config
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

pub fn evaluate(config: Config, input: FlowState) {
    debug!(
        "config: {:?}, flow: {:?}",
        config.version_name, input.unique_id
    );
    //
    // ASSUMPTION -- non stateful, so need/expect to iter states
    //

    for route in config.flow_routes.iter() {
        debug!("flow_route_name: {:?}", route.flow_route_name);
        for conditions in route.flow_conditional_matches.iter() {
            debug!(
                "\troute_condition_name: {:?}",
                conditions.route_condition_name
            );
            for state in input.state_transitions.iter() {
                eval_condition_any(conditions, &state);
            }
        }
    }
}

fn eval_condition_any(conditions: &ConditionMatches, states: &Vec<State>) {
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
    let bool_result = false;

    if cond.param_type == "STRING" {
        eval_string_expression(value, &cond.param_name, &cond.param_match);
    } else if cond.param_type == "NUM" {
        let value = value.parse::<f64>();
        match value {
            Ok(val) => {
                debug!("\tmatch value: {:?}", val);
                let bool_result = eval_num_expression(val, &cond.param_name, &cond.param_match);
            }
            Err(error) => {
                error!(
                    "NUM type given for value that cannot be parsed as number: {:?}",
                    error
                );
                let bool_result = false;
            }
        };
    } else {
        error!(
            "type information for match condition error: {:?}",
            cond.param_type
        );
    };
    bool_result
}

fn eval_string_expression(value: &String, key: &String, expr: &String) -> bool {
    debug!("\t\teval_string_expression: {:?}", expr);
    let bool_result = false;
    let key = "$string";
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
    let bool_result = false;
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
