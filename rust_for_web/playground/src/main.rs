use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, put},
    Router,
};
use playground::{
    models::{self, Plant},
    services, AppState, PlantRegister,
};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let plant_register: PlantRegister = Arc::new(Mutex::new(HashMap::new()));

    // insert some plants into the register
    let mut m_plant_register = plant_register.lock().unwrap();
    let id1 = Uuid::new_v4().to_string();
    m_plant_register.insert(
        id1.clone(),
        Plant {
            id: id1,
            name: "Tomato".to_string(),
            description: Some("A red fruit".to_string()),
        },
    );

    let id2 = Uuid::new_v4().to_string();
    m_plant_register.insert(
        id2.clone(),
        Plant {
            id: id2,
            name: "Cucumber".to_string(),
            description: Some("A green fruit".to_string()),
        },
    );

    drop(m_plant_register);

    let garden_register = Arc::new(Mutex::new(models::Garden {
        name: "My Garden".to_string(),
        size: 5,
        fields: vec![None; 25],
    }));

    let app_state = AppState {
        plant_register: plant_register.clone(),
        garden_register: garden_register.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route("/garden", get(services::get_garden))
        .route("/garden/field/:id", put(services::update_field))
        .route(
            "/plants",
            get(services::get_plants).post(services::add_plant),
        )
        .layer(cors)
        .with_state(app_state);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
