use actix_web::{web, HttpResponse, Responder};
use crate::services::notification::NotificationService;
use crate::models::notification::{Notification, NotificationConfig};
use crate::auth::AuthUser;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(get_notifications))
            .route("", web::post().to(create_notification))
            .route("/{id}", web::get().to(get_notification))
            .route("/{id}", web::put().to(update_notification))
            .route("/{id}", web::delete().to(delete_notification))
            .route("/{id}/test", web::post().to(test_notification))
    );
}

async fn get_notifications(
    service: web::Data<NotificationService>,
    user: AuthUser,
) -> impl Responder {
    match service.get_user_notifications(user.id).await {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn create_notification(
    service: web::Data<NotificationService>,
    user: AuthUser,
    config: web::Json<NotificationConfig>,
) -> impl Responder {
    match service.create_notification(user.id, config.into_inner()).await {
        Ok(notification) => HttpResponse::Created().json(notification),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn get_notification(
    service: web::Data<NotificationService>,
    user: AuthUser,
    id: web::Path<i64>,
) -> impl Responder {
    match service.get_notification(id.into_inner(), user.id).await {
        Ok(Some(notification)) => HttpResponse::Ok().json(notification),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_notification(
    service: web::Data<NotificationService>,
    user: AuthUser,
    id: web::Path<i64>,
    config: web::Json<NotificationConfig>,
) -> impl Responder {
    match service.update_notification(id.into_inner(), user.id, config.into_inner()).await {
        Ok(notification) => HttpResponse::Ok().json(notification),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

async fn delete_notification(
    service: web::Data<NotificationService>,
    user: AuthUser,
    id: web::Path<i64>,
) -> impl Responder {
    match service.delete_notification(id.into_inner(), user.id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn test_notification(
    service: web::Data<NotificationService>,
    user: AuthUser,
    id: web::Path<i64>,
) -> impl Responder {
    match service.get_notification(id.into_inner(), user.id).await {
        Ok(Some(notification)) => {
            match service.test_notification(&notification).await {
                Ok(msg) => HttpResponse::Ok().json(serde_json::json!({ "message": msg })),
                Err(e) => HttpResponse::BadRequest().body(e.to_string()),
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
