#[allow(dead_code)]
pub mod values {
    pub const AVALUE: &str = "a test string";

    pub const VERSION: &str = "0.1.1";

    pub const HELP: &str = r#"
*** Just-flow-core © 2022 ***
*   Fraign Analytics LLC    *
*   222 S Meramec Ave.,     *
*   St. Louis, MO           *
*   63105 USA               *
*   fraignanalytics.com     *
*****************************

The just-flow-core is a Python binary module for fast execution of automata on 
sequential state data. In order to .process() & evaluate the automata, it 
requires a String describing the automata config and a String decribing the 
"flow" or sequence of states. Examples of these Strings and their required 
format can be found by calling .example_configuration() & .example_flow().

With this approach a wide variety of data can be evaluated and appropriate 
resulting action returned. One example would be the evaluation of what stage
of flight an aircraft is currently in and what instructions should be passed 
back to the aicraft/simulator/pilot as the next actions. This could be 
identifying that the aircraft is on the descent to land and returning the before
landing checklist to the pilot. Determining the next action requires knowledge 
of the prior actions--hence the need for an automata.

In order to demonstrate what just-flow-core does, let's set up an example for a
sequence flow of five states labeled 0-4. Each of these events has features
associated with it that can be strings, labels, or parameters. Here we will use
airspeed (A) & height (H), which are both floats. A valid flow of states would 
be:
                      states
feature   0      1      2      3      4
          | ---- | ---- | ---- | ---- |
A         0     150    250    250    150
H        420    1500   2500   1500   600

A single automata (just-flow-core allows for many) to identify the current 
flight state and required checklist could be as follows, where FS is the flight
state and CL the checklist that the pilot will need next:

State        Entry Cond.              Action                  Next States.
START        A <= 0 && H == 420       FS = "Startup"          climb, abort
                                      CL = "before taxi""
climb        A > 120 && H > 820       FS = "takeoff climb"    cruise
                                      CL = "cruise"
cruise       A > 200 && H > 2000      FS = "low cruise"       descent
                                      CL = "descent"
descent      A < 160 && H < 1500      FS = "descent to land"  END
END          A < 60 && H <= 600       FS = "shutdown"        
                                      CL = "shutdown"

Note that just-flow-core allows for any number of states, conditions, and 
actions for each automata. An automata defining the first two of these states 
(START & climb) would need the following config file:

{
    "version_name": "uniqueIdString",
    "stateful": false,
    "flow_routes": [
        {
            "flow_route_name": "help demonstration",
            "flow_conditional_matches": [
                {
                    "route_condition_name": "START",
                    "match_type": "START",
                    "match_condition_type": "ANY",
                    "match_conditions": [
                        {
                            "param_name": "$A",
                            "param_key": "A",
                            "param_type": "NUM",
                            "param_match": "$airspeed = 0"
                        },
                        {
                            "param_name": "$H",
                            "param_key": "H",
                            "param_type": "NUM",
                            "param_match": "$H = 420"
                        }
                    ],
                    "match_actions": [
                        {
                            "action_key": "FS",
                            "action_type": "f64",
                            "action_value": "startup"
                        },
                        {
                            "action_key": "CL",
                            "action_type": "f64",
                            "action_value": "before taxi"
                        }
                    ],
                    "next_available_matches": [
                        "climb", "abort"
                    ]
               },
               {
                   "route_condition_name": "climb",
                   "match_type": "EVAL",
                   "match_condition_type": "ANY",
                   "match_conditions": [
                       {
                           "param_name": "$A",
                           "param_key": "A",
                           "param_type": "NUM",
                           "param_match": "$airspeed > 120"
                       },
                       {
                           "param_name": "$H",
                           "param_key": "H",
                           "param_type": "NUM",
                           "param_match": "$H > 820"
                        }
                   ],
                   "match_actions": [
                       {
                           "action_key": "FS",
                           "action_value": "takeoff climb"
                       },
                       {
                           "action_key": "CL",
                           "action_value": "cruise"
                       }
                   ],
                   "next_available_matches": [
                       "cruise"
                   ]
               }
           ]
       }
   ]
}

The flow state to be evaluated against this config may look like:
A         0     150    250    250    150
H        420    1500   2500   1500   600

{
    "unique_id": "some_uid",
    "state_transitions": [
        [
	    {
		"param_name": "A",
		"param_value": "0"
	    },
	    {
		"param_name": "H",
		"param_value": "420"
	    }
	],
        [
	    {
		"param_name": "A",
		"param_value": "150"
	    },
	    {
		"param_name": "H",
		"param_value": "1500"
	    }
	],
        [
	    {
		"param_name": "A",
		"param_value": "250"
	    },
	    {
		"param_name": "H",
		"param_value": "2500"
	    }
	]
    ]
}


    "#;

    pub const STATE: &str = r#"
	    {
		"param_name": "airspeed",
		"param_value": "val"
	    }
        "#;

    pub const FLOW: &str = r#"
{
    "unique_id": "some_uid",
    "state_transitions": [
        [
	    {
		"param_name": "altitude",
		"param_value": "5000"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "320"
	    }
	],
        [
	    {
		"param_name": "altitude",
		"param_value": "5050"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "321"
	    }
	]
    ]
}"#;

    pub const ACTIONS: &str = r#"
            {
                "action_key": "dref value",
                "action_value": "12366"
            }
        "#;

    pub const ACTIONSTATE: &str = r#"
         {
            "unique_id": "someuuid",
            "action_transitions": [
                [
                    {
                        "action_key": "dref val 1",
                        "action_value": "2222"
                    },
                    {
                        "action_key": "dref val 1",
                        "action_value": "2222"
                    }
                ],
                [
                    {
                        "action_key": "dref val 1",
                        "action_value": "2222"
                    },
                    {
                        "action_key": "dref val 1",
                        "action_value": "2222"
                    }
                ]
             ]
        }
        "#;

    pub const MATCHCONDITION: &str = r#"
         {
            "param_name": "$airspeed",
            "param_key": "airspeed",
            "param_type": "NUM",
            "param_match": "$airspeed > 150"
         }
        "#;

    pub const CONDITIONMATCHES: &str = r#"
        {
            "route_condition_name": "START",
            "match_type": "START",
            "match_condition_type": "ANY",
            "match_conditions": [
                {
                    "param_name": "$airspeed",
                    "param_key": "airspeed",
                    "param_type": "NUM",
                    "param_match": "$airspeed > 140"
                },
                {
                    "param_name": "$altitude",
                    "param_key": "altitude",
                    "param_type": "NUM",
                    "param_match": "$altitude > 1000"
                }
            ],
            "match_actions": [
                {
                    "action_key": "dref_1",
                    "action_type": "f64",
                    "action_value": "190"
                },
                {
                    "action_key": "dref_2",
                    "action_type": "f64",
                    "action_value": "5050"
                }
            ],
            "next_available_matches": [
                "takeoff_climb_1", "emergency_1"
            ]
}"#;

    pub const ROUTEFLOW: &str = r#"
        {
            "flow_route_name": "evaluate entire event flow",
            "flow_conditional_matches": [
                {
                    "route_condition_name": "takeoff_1",
                    "match_type": "START",
                    "match_condition_type": "ANY",
                    "match_conditions": [
                        {
                            "param_name": "$airspeed",
                            "param_key": "airspeed",
                            "param_type": "NUM",
                            "param_match": "$airspeed > 140"
                        },
                        {
                        "param_name": "$altitude",
                        "param_key": "altitude",
                        "param_type": "NUM",
                        "param_match": "$altitude > 1000"
                        }
                    ],
                    "match_actions": [
                        {
                            "action_key": "dref_1",
                            "action_type": "f64",
                            "action_value": "190"
                        },
                        {
                            "action_key": "dref_2",
                            "action_type": "f64",
                            "action_value": "5050"
                        }
                    ],
                    "next_available_matches": [
                        "takeoff_climb_1", "emergency_1"
                    ]
                },
                {
                    "route_condition_name": "takeoff_climb_1",
                    "match_type": "EVAL",
                    "match_condition_type": "ANY",
                    "match_conditions": [
                        {
                            "param_name": "$airspeed",
                            "param_key": "airspeed",
                            "param_type": "NUM",
                            "param_match": "$airspeed > 150"
                        },
                        {
                        "param_name": "$altitude",
                        "param_key": "altitude",
                        "param_type": "NUM",
                        "param_match": "$altitude > 2000"
                        }
                    ],
                    "match_actions": [
                        {
                            "action_key": "dref_1",
                            "action_type": "f64",
                            "action_value": "190"
                        },
                        {
                            "action_key": "dref_2",
                            "action_type": "f64",
                            "action_value": "5050"
                        }
                    ],
                    "next_available_matches": [
                        "takeoff_climb_2", "emergency_1"
                    ]
                }
            ]
}"#;

    pub const FLOWSTATE: &str = r#"
{
    "unique_id": "some_uid",
    "state_transitions": [
        [
	    {
		"param_name": "altitude",
		"param_value": "5000"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "320"
	    }
	],
        [
	    {
		"param_name": "altitude",
		"param_value": "5050"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "321"
	    }
	]
    ]
}"#;

    pub const CONFIG: &str = r#"
        {
            "version_name": "version1",
            "stateful": false,
            "flow_routes": [
                {
                    "flow_route_name": "evaluate entire event flow",
                    "flow_conditional_matches": [
                        {
                            "route_condition_name": "START",
                            "match_type": "START",
                            "match_condition_type": "ANY",
                            "match_conditions": [
                               {
                                    "param_name": "$airspeed",
                                    "param_key": "airspeed",
                                    "param_type": "NUM",
                                    "param_match": "$airspeed > 140"
                               },
                               {
                                    "param_name": "$altitude",
                                    "param_key": "altitude",
                                    "param_type": "NUM",
                                    "param_match": "$altitude > 1000"
                               }
                            ],
                            "match_actions": [
                                {
                                    "action_key": "dref_1",
                                    "action_type": "f64",
                                    "action_value": "190"
                                },
                                {
                                   "action_key": "dref_2",
                                  "action_type": "f64",
                                 "action_value": "5050"
                                }
                           ],
                           "next_available_matches": [
                               "takeoff_climb_1", "emergency_1"
                           ]
                    },
                    {
                        "route_condition_name": "takeoff_climb_1",
                        "match_type": "EVAL",
                        "match_condition_type": "ANY",
                        "match_conditions": [
                             {
                                "param_name": "$airspeed",
                                "param_key": "airspeed",
                                "param_type": "NUM",
                                "param_match": "$airspeed > 150"
                             },
                             {
                                "param_name": "$altitude",
                                "param_key": "altitude",
                                "param_type": "NUM",
                                "param_match": "$altitude > 2000"
                             }
                        ],
                        "match_actions": [
                            {
                                "action_key": "dref_1",
                                "action_value": "190"
                            },
                            {
                                "action_key": "dref_2",
                                "action_value": "5050"
                            }
                        ],
                        "next_available_matches": [
                            "takeoff_climb_2", "emergency_1"
                        ]
                    }
                ]
          }
    ]

}"#;

    ///////////////////
    // bad constants //
    ///////////////////

    // missing unique_id
    pub const BADFLOWSTATE: &str = r#"
{
    "state_transitions": [
        [
	    {
		"param_name": "altitude",
		"param_value": "5000"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "320"
	    }
	],
        [
	    {
		"param_name": "altitude",
		"param_value": "5050"
	    },
	    {
		"param_name": "airspeed",
		"param_value": "321"
	    }
	]
    ]
}"#;

    // missing match_key
    pub const BADCONFIG: &str = r#"
        {
            "version_name": "version1",
            "stateful": false,
            "flow_routes": [
                {
                    "flow_route_name": "evaluate entire event flow",
                    "flow_conditional_matches": [
                        {
                            "route_condition_name": "START",
                            "match_type": "START",
                            "match_condition_type": "ANY",
                            "match_conditions": [
                               {
                                    "param_name": "$airspeed",
                                    "param_type": "NUM",
                                    "param_match": "$airspeed > 140"
                               },
                               {
                                    "param_name": "$altitude",
                                    "param_key": "altitude",
                                    "param_type": "NUM",
                                    "param_match": "$altitude > 1000"
                               }
                            ],
                            "match_actions": [
                                {
                                    "action_key": "dref_1",
                                    "action_type": "f64",
                                    "action_value": "190"
                                },
                                {
                                   "action_key": "dref_2",
                                  "action_type": "f64",
                                 "action_value": "5050"
                                }
                           ],
                           "next_available_matches": [
                               "takeoff_climb_1", "emergency_1"
                           ]
                    },
                    {
                        "route_condition_name": "takeoff_climb_1",
                        "match_type": "EVAL",
                        "match_condition_type": "ANY",
                        "match_conditions": [
                             {
                                "param_name": "$airspeed",
                                "param_key": "airspeed",
                                "param_type": "NUM",
                                "param_match": "$airspeed > 150"
                             },
                             {
                                "param_name": "$altitude",
                                "param_key": "altitude",
                                "param_type": "NUM",
                                "param_match": "$altitude > 2000"
                             }
                        ],
                        "match_actions": [
                            {
                                "action_key": "dref_1",
                                "action_value": "190"
                            },
                            {
                                "action_key": "dref_2",
                                "action_value": "5050"
                            }
                        ],
                        "next_available_matches": [
                            "takeoff_climb_2", "emergency_1"
                        ]
                    }
                ]
          }
    ]

}"#;

    //////////////////////
    // end of constants //
    //////////////////////
}
