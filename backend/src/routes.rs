use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{directions, health, reports, route_score, sos},
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        // Reports
        .route(
            "/reports",
            post(reports::create_report).get(reports::get_reports),
        )
        .route("/reports/:id/upvote", post(reports::upvote_report))
        .route("/reports/:id/downvote", post(reports::downvote_report))
        // Route safety score
        .route("/directions", get(directions::get_directions))
        .route("/route-score", post(route_score::calculate_route_score))
        // SOS & emergency contacts
        .route("/sos/trigger", post(sos::trigger_sos))
        .route(
            "/sos/contacts",
            post(sos::add_contact).get(sos::get_contacts),
        )
        .route(
            "/sos/contacts/:id",
            axum::routing::delete(sos::delete_contact),
        )
        .route("/sos/subscribe", post(sos::subscribe_push))
        .route("/sos/invite/:token", get(sos::get_invite_info))
        // Config (exposes public VAPID key and Maps key to frontend)
        .route("/config", get(health::get_public_config))
}
