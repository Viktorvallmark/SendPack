use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, post};
use crate::model::{ModelController, Ticket, CreateTicket};

use crate::{Result};


//++ Handling REST calls
#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub async fn rest_routes(controller: ModelController) -> Router {
    let app_state = AppState { mc: controller};
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_tickets))
        .with_state(app_state)
}

async fn create_ticket(State(controller): State<ModelController>, Json(ticket_creation): Json<CreateTicket>) -> Result<Json<Ticket>> {

    println!("->> {:<12} - create_ticket ", "HANDLER");

    let ticket = controller.create_ticket(ticket_creation).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(controller): State<ModelController>) -> Result<Json<Vec<Ticket>>> {

    println!("->> {:<12} - list_tickets ", "HANDLER");

    let tickets = controller.list_ticket().await?;

    Ok(Json(tickets))
}

async fn delete_tickets(State(controller): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {

    println!("->> {:<12} - delete_tickets ", "HANDLER");

    let ticket = controller.delete_ticket(id).await?;

    Ok(Json(ticket))
}


//-- Handling REST calls