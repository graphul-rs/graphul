use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    pub user_set: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
}

pub fn app_state() -> Arc<AppState> {
    // Set up application state for use with share_state.
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    Arc::new(AppState { user_set, tx })
}
