#[allow(dead_code)]
pub mod values {
    pub const AVALUE: &str = "a test string";

    pub const HELP: &str = r#"
Just-flow 
2021 Fraign Analytics LLC    

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
