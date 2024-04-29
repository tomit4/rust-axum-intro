use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use crate::ctx::Ctx;
// use axum::extract::{State, Path, FromRef};
use axum::extract::{State, Path};
use axum::{Json, Router};
use axum::routing::{post, delete};

// region:  --- REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("->> {:12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:12} - create_ticket", "HANDLER");

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))

}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:15} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}

// requres axum features = ["macros"] in Cargo.toml
// #[derive(Clone, FromRef)]
// struct AppState {
    // mc: ModelController,
// }

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
        // .with_state(app_state)
}
// endregion:  --- REST Handlers
