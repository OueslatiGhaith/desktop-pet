use crate::animation_states::cave_chaos;
use bevy::{prelude::*, utils::HashMap};
use rand::prelude::*;

#[derive(Debug)]
pub struct AnimationState {
    pub offset: [i32; 2],
    pub size: [f32; 2],
    pub translate: Option<[i32; 2]>,
    pub sprite_sheet: AnimationStateSpriteSheet,
    pub transitions_to: Vec<AnimationStateTransition>,
}

#[derive(Debug)]
pub struct AnimationStateSpriteSheet {
    pub file_name: String,
    pub columns: usize,
    pub rows: usize,
    pub frame_duration: f32,
}

#[derive(Debug)]
pub struct AnimationStateTransition {
    pub target: String,
    pub probability: f32,
}

#[derive(Resource)]
pub struct AnimationStateMachine {
    states: HashMap<String, AnimationState>,
    pub current_state: String,
    pub is_heading_right: bool,
}

impl AnimationStateMachine {
    pub fn new() -> Self {
        let mut states = HashMap::new();

        let initial_state = cave_chaos::insert_states(&mut states);

        Self {
            states,
            current_state: initial_state,
            is_heading_right: true,
        }
    }

    pub fn get_current_state(&self) -> &AnimationState {
        self.states.get(&self.current_state).unwrap()
    }

    pub fn get_states(&self) -> &HashMap<String, AnimationState> {
        &self.states
    }

    pub fn advance_state(&mut self) {
        let current_state = self.get_current_state();
        let mut rng = rand::thread_rng();
        let mut total_probability = 0.0;
        for transition in &current_state.transitions_to {
            total_probability += transition.probability;
        }
        let mut random_number = rng.gen_range(0.0..total_probability);
        for transition in &current_state.transitions_to {
            if random_number < transition.probability {
                self.current_state = transition.target.clone();
                break;
            } else {
                random_number -= transition.probability;
            }
        }
    }
}
