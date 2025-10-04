use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(name = "添加一个订阅者",
    skip(form, pool), fields(
    subscriber_email=%form.email,
    subscriber_name=%form.name
    ))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "保存一个订阅者", skip(form, pool))]
pub async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("执行SQL失败： {:?}", e);
        e
    })?;
    Ok(())
}
