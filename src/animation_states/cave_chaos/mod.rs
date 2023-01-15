use bevy::utils::HashMap;
use serde_json::Value;

use super::state_machine::{AnimationState, AnimationStateSpriteSheet, AnimationStateTransition};

/// inserts all states inside the states.json file into the given HashMap
/// returns the name of the initial state
pub fn insert_states(hash_map: &mut HashMap<String, AnimationState>) -> String {
    let data = include_str!("./states.json");
    let v: Value = serde_json::from_str(data).expect("Failed to parse states.json");

    for state in v.as_array().expect("states.json is not an array") {
        let mut transitions = Vec::new();
        for transition in state["transitions_to"]
            .as_array()
            .expect("transitions_to is not an array")
        {
            let target = transition["target"]
                .as_str()
                .expect("target is not a string")
                .to_string();
            let probability = transition["probability"]
                .as_f64()
                .expect("probability is not a float") as f32;
            transitions.push(AnimationStateTransition {
                target,
                probability,
            });
        }

        let animation_state = AnimationState {
            offset: [
                state["dims"].as_array().expect("dims is not an array")[0]
                    .as_i64()
                    .expect("dims[0] is not an integer") as i32,
                state["dims"].as_array().expect("dims is not an array")[1]
                    .as_i64()
                    .expect("dims[1] is not an integer") as i32,
            ],
            size: [
                state["dims"].as_array().expect("dims is not an array")[2]
                    .as_f64()
                    .expect("dims[2] is not a float") as f32,
                state["dims"].as_array().expect("dims is not an array")[3]
                    .as_f64()
                    .expect("dims[3] is not a float") as f32,
            ],
            translate: state["move"].as_array().map(|translate| {
                [
                    translate[0].as_i64().expect("move[0] is not an integer") as i32,
                    translate[1].as_i64().expect("move[1] is not an integer") as i32,
                ]
            }),
            sprite_sheet: state["sprite"]
                .as_object()
                .map(|sprite_sheet| AnimationStateSpriteSheet {
                    file_name: sprite_sheet["file_name"]
                        .as_str()
                        .expect("file_name is not a string")
                        .to_string(),
                    columns: sprite_sheet["columns"]
                        .as_u64()
                        .expect("columns is not an integer") as usize,
                    rows: sprite_sheet["rows"]
                        .as_u64()
                        .expect("rows is not an integer") as usize,
                    frame_duration: sprite_sheet["frame_duration"]
                        .as_f64()
                        .expect("frame_duration is not a float")
                        as f32,
                })
                .expect("sprite_sheet is not an object"),
            transitions_to: transitions,
        };

        hash_map.insert(
            state["state_name"]
                .as_str()
                .expect("state_name is not a string")
                .to_string(),
            animation_state,
        );
    }

    v.as_array().expect("states.json is not an array")[0]["state_name"]
        .as_str()
        .expect("state_name is not a string")
        .to_string()
}
