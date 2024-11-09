use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use sqlx::types::chrono;
use sqlx::types::chrono::Local;
use crate::AppState;
use crate::utils::pagination::Pagination;

#[derive(Debug, Deserialize)]
pub struct CreateTicketReq {
    pub title: String,
    pub description: String,
    pub body: String,
    pub status: u8,
}


pub async fn root_handler(State(_state): State<AppState>) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn create_handle(
    State(state): State<AppState>,
    Json(req): Json<CreateTicketReq>,
) -> impl IntoResponse {
    tracing::info!("Create user: {:?}", req);

    let insert_sql = r#"
        INSERT INTO tickets (title, description, body, status)
        VALUES (?, ?, ?, ?)
        "#;
    let res = sqlx::query(insert_sql).bind(req.title).bind(req.description).bind(req.body).bind(req.status).execute(&state.pool).await.unwrap();

    tracing::info!("last id: {:?}", res.last_insert_id());

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Ticket created successfully",
            "id": res.last_insert_id()
            }))
    )
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserReq {
    pub title: String,
    pub description: String,
    pub status: u8,
}

pub async fn update_handle(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
    Json(req): Json<UpdateUserReq>,
) -> impl IntoResponse {
    tracing::info!("Update user: {} with {:?}", ticket_id, req);

    let update_sql = r#"
        UPDATE tickets
        SET title = ?, description = ?, status = ?
        WHERE id = ?
        "#;

    sqlx::query(update_sql).bind(req.title).bind(req.description).bind(req.status).bind(ticket_id).execute(&state.pool).await.unwrap();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Ticket updated successfully",
            }))
    )
}

pub async fn delete_handle(State(state): State<AppState>, Path(ticket_id): Path<u64>) -> impl IntoResponse {
    tracing::info!("Delete user: {}", ticket_id);
    let delete_sql = r#"
        DELETE FROM tickets WHERE id = ?
        "#;
    let _res = sqlx::query(delete_sql).bind(ticket_id).execute(&state.pool).await.unwrap();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Ticket deleted successfully",
            }))
    )
}


#[derive(Debug, Serialize, FromRow)]
pub struct Ticket {
    title: String,
    body: Option<String>,
    description: Option<String>,
    status: i64,
}

pub async fn get_handle(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> impl IntoResponse {
    tracing::info!("Get user: {}", ticket_id);
    let get_sql = r#"
        SELECT title,body,description,status FROM tickets WHERE id = ?
        "#;

    let ticket: Option<Ticket> = sqlx::query_as(get_sql).bind(ticket_id).fetch_optional(&state.pool).await.unwrap();
    if let Some(ticket) = ticket {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "ok",
                "data": ticket
            }))
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "message": "Ticket not found"
            }))
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TicketListItem {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: i64,
    pub created_at: Option<chrono::DateTime<Local>>,
    pub update_at: Option<chrono::DateTime<Local>>,
}

pub async fn list_handle(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    let list_sql = "SELECT id, title, description, status, created_at, update_at FROM tickets WHERE 1=1 ";

    let mut pagination: Pagination = query.page.into();
    let limit = query.limit.unwrap_or(1);
    pagination.set_size(limit);
    let (offset, limit) = pagination.compute();

    let mut query_builder = sqlx::QueryBuilder::new(list_sql);
    query_builder.push(" LIMIT ").push_bind(limit);
    query_builder.push(" OFFSET ").push_bind(offset);

    let rows = query_builder.build_query_as::<TicketListItem>().fetch_all(&state.pool).await.unwrap();

    (StatusCode::OK, Json(serde_json::json!({
        "message": "ok",
        "data": rows
    })))
}
