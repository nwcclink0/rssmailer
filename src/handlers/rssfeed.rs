use std::str::FromStr;

use super::ResponseError;
use crate::db::rssfeed::add_rssfeed;
use crate::db::rssfeed::delete_rssfeed;
use crate::db::rssfeed::fetch_account_rssfeeds;
use crate::db::rssfeed::RssFeed;
use crate::handlers::account::get_token_from_request;
use crate::handlers::authentication::decode_token;

use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use strum::{AsRefStr, AsStaticStr, EnumString, IntoStaticStr};

#[derive(Debug, Eq, PartialEq, EnumString, AsRefStr, AsStaticStr, IntoStaticStr)]
pub enum RssFeedAction {
    #[strum(to_string = "add")]
    Add,

    #[strum(to_string = "delete")]
    Delete,

    #[strum(to_string = "get")]
    Get,
}

#[derive(Serialize, Deserialize)]

pub struct RssFeedRequest {
    pub action: String,
    pub token: String,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct RssFeedResponse {
    pub rssfeeds: Vec<RssFeed>,
    pub status: u16,
}

#[post("/rssfeed/{user_id}")]
pub async fn post_rssfeeds(
    context: web::Data<PgPool>,
    json: web::Json<RssFeedRequest>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = context;
    let uuid = uuid::Uuid::from_str(id.as_str()).unwrap();
    let token = get_token_from_request(&req).unwrap();
    let decode_id = decode_token(token).await.unwrap();
    if decode_id != uuid.to_string() {
        let empty_response = RssFeedResponse {
            rssfeeds: Vec::new(),
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Unauthorized().json(empty_response)
    } else {
        if json.action == RssFeedAction::Add.as_ref() {
            add_rssfeed(&pool, uuid, json.link.clone()).await.unwrap();
            let rssfeeds = fetch_account_rssfeeds(&pool, uuid).await.unwrap();
            let response = RssFeedResponse {
                rssfeeds,
                status: ResponseError::Success as u16,
            };
            HttpResponse::Ok().json(response)
        } else if json.action == RssFeedAction::Delete.as_ref() {
            delete_rssfeed(&pool, uuid, json.link.clone())
                .await
                .unwrap();
            let rssfeeds = fetch_account_rssfeeds(&pool, uuid).await.unwrap();
            let response = RssFeedResponse {
                rssfeeds,
                status: ResponseError::Success as u16,
            };
            HttpResponse::Ok().json(response)
        } else if json.action == RssFeedAction::Get.as_ref() {
            let rssfeeds = fetch_account_rssfeeds(&pool, uuid).await.unwrap();
            let response = RssFeedResponse {
                rssfeeds,
                status: ResponseError::Success as u16,
            };
            HttpResponse::Ok().json(response)
        } else {
            let empty_response = RssFeedResponse {
                rssfeeds: Vec::new(),
                status: ResponseError::Failure as u16,
            };
            HttpResponse::Ok().json(empty_response)
        }
    }
}
