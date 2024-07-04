use serde::{Deserialize, Serialize};
use ahash::{HashMap, HashMapExt};

#[derive(Serialize, Deserialize)]
pub struct Config {
    current_term: u64,
    voted_for: Option<String>,
}

enum Role {
    Undef,
    Follower,
    Candidate,
    Leader,
}

struct LeaderState {
    pub next_index: HashMap<String, u64>, // node id/index
    pub match_index: HashMap<String, u64>, // node id/index
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
}