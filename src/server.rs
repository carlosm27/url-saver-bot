use axum::{
    routing::get,
    http::StatusCode,
    response::Redirect,
    Json, Router,
    extract::Path,
};

use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use serde_json::json;


mod data;
use crate::data::DATA;

pub fn start_server() {
    
    tracing_subcriber::fmt::new();
    
    let app = Router::new()
        .route("/redirect", get(redirect));
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();    
}

pub fn redirect(Path(id): Path<String>) -> Result<Redirect, (StatusCode, String)> {
    
    let stored_url = DATA.lock().unwrap();
    if stored_url.get(&id) {
        Ok(Redirect::to(&stored_url.url))
        
    }
    
}
