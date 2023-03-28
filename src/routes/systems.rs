use axum::{
    extract::{State},
    response::IntoResponse,
    Json,
};

use crate::model::{AppState, System};

pub async fn get_systems(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let systems_map = app_state.systems.read().unwrap();

    let systems: Vec<System> = systems_map.values().cloned().collect();
    Json(systems)
}

pub async fn create_system(
    State(app_state): State<AppState>,
    address: String
) -> impl IntoResponse {
    let mut systems_map = app_state.systems.write().unwrap();

    let system = System::new(address);

    systems_map.insert(system.get_address().to_string(), system);
}