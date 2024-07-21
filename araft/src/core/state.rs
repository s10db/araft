use serde::{Deserialize, Serialize};
use ahash::{HashMap, HashMapExt};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    current_term: u64,
    voted_for: Option<String>,
}

#[derive(Clone)]
pub enum Role {
    Undef,
    Follower,
    Candidate,
    Leader,
}

struct LeaderState {
    next_index: HashMap<String, u64>, // node id/index
    match_index: HashMap<String, u64>, // node id/index
}

struct State {
    commit_index: u64,
    last_applied: u64,

    leader_state: Option<LeaderState>,

    current_role: Role,
}

pub struct Handle {
    config: Config,
    state: State,
}

impl Handle {
    pub fn new(config: Config) -> Self {
        Self { 
            config,
            state: State {
                commit_index: 0,
                last_applied: 0,
                leader_state: None,
                current_role: Role::Follower,
            }
        }
    }

    pub fn role(&self) -> Role {
        self.state.current_role.clone()
    }
}