use crate::models::car::Car;
use leptos::*;
use leptos::logging::log;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use uuid::Uuid;

pub struct DBError {}

#[server(AppendCar)]
pub async fn append_car(make: String, model: String, year: u16, color: String, price: f32) -> Result<Car, ServerFnError> {
    let id = Uuid::new_v4().to_string();
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();
    let _res: Vec<Car> =
        sqlx::query_as("INSERT INTO car (id, make, model, year, color, price) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&id)
            .bind(&make)
            .bind(&model)
            .bind(year)
            .bind(&color)
            .bind(price)
            .fetch_all(&*pool)
            .await
            .map_err(|err| {
                log!("error: {:?}", err);
                err
            })?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(Car {
        id,
        make,
        model,
        year,
        color,
        price,
    })
}

#[server(ReplaceCar)]
pub async fn replace_car(car: Car) -> Result<(), ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    sqlx::query("UPDATE car SET make = $1, model = $2, year = $3, color = $4, price = $5 WHERE ID = $6")
        .bind(&car.make)
        .bind(&car.model)
        .bind(car.year)
        .bind(&car.color)
        .bind(car.price)
        .bind(&car.id)
        .execute(&*pool)
        .await
        .map_err(|err| {
            log!("error: {:?}", err);
            err
        })?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(())
}


#[server(RemoveCar)]
pub async fn remove_car(id: String) -> Result<(), ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();

    sqlx::query("DELETE FROM car WHERE ID = $1")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|err| {
            log!("error: {:?}", err);
            err
        })?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(())
}

#[server(AllCars)]
pub async fn all_cars() -> Result<Vec<Car>, ServerFnError> {
    let conn: Data<Pool<Sqlite>> = extract().await?;
    let pool = conn.into_inner();
    let res: Vec<Car> = sqlx::query_as(
        "SELECT
            id, make, model, year, color, price
        FROM car
        ORDER BY id",
    )
    .fetch_all(&*pool)
    .await?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(res)
}
