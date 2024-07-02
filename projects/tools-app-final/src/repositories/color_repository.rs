use crate::models::color::Color;
use leptos::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use uuid::Uuid;

pub struct DBError {}

#[server(AppendColor)]
pub async fn append_color(name: String, hex_code: String) -> Result<Color, ServerFnError> {
    let id = Uuid::new_v4().to_string();
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();
    let _res: Vec<Color> =
        sqlx::query_as("INSERT INTO color (id, name, hex_code) VALUES ($1, $2, $3)")
            .bind(&id)
            .bind(&name)
            .bind(&hex_code)
            .fetch_all(&*pool)
            .await?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(Color { id, name, hex_code })
}

#[server(RemoveColor)]
pub async fn remove_color(id: String) -> Result<(), ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    sqlx::query("DELETE FROM color WHERE ID = $1")
        .bind(id)
        .execute(&*pool)
        .await?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(())
}

#[server(AllColors)]
pub async fn all_colors() -> Result<Vec<Color>, ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();
    let res: Vec<Color> = sqlx::query_as(
        "SELECT
            id, name, hex_code
        FROM color
        ORDER BY id",
    )
    .fetch_all(&*pool)
    .await?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(res)
}
