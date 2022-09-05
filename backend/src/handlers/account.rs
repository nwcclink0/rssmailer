use super::*;
use std::str::FromStr;

use crate::db::account::*;
use crate::handlers::authentication::create_token;
use crate::handlers::authentication::decode_token;
use crate::services::mailer::get_smtp_info;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use otpauth::TOTP;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{header, MultiPart, SinglePart},
    SmtpTransport, Transport,
};
use maud::html;

#[derive(Serialize, Deserialize)]
pub struct EditAccountRequest {
    pub email: String,
    pub nickname: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddAccountRequest {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountAuthRequest {
    pub email: String,
    pub provider: u32,
    pub auth_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountAuthResponse {
    pub token: String,
    pub status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AccountSendVerifyEmailCodeRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountSendVerifyEmailCodeResponse {
    pub code: u32,
    pub status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct AccountVerifyEmailCodeResponse {
    pub status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct PostAccountResponseAccount {
    pub account: AccountInfo,
    pub status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct PostAccountResponse {
    pub status: i32,
}

#[post("/account/{user_id}")]
async fn post_account(
    context: web::Data<PgPool>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = context;
    let uuid = uuid::Uuid::from_str(id.as_str()).unwrap();
    let token = get_token_from_request(&req).unwrap();
    let decode_id = decode_token(token).await.unwrap();
    if decode_id != uuid.to_string() {
        let response = PostAccountResponseAccount {
            account: get_empty_account_info().to_owned(),
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Unauthorized().json(response)
    } else {
        let account = fetch_account_info(&pool, uuid).await.unwrap();
        let response = PostAccountResponseAccount {
            account: account.to_owned(),
            status: ResponseError::Success as u16,
        };
        HttpResponse::Ok().json(response)
    }
}

#[post("/account/add")]
async fn post_add_account(
    context: web::Data<PgPool>,
    json: web::Json<AddAccountRequest>,
) -> impl Responder {
    let pool = context;
    let uuid = Uuid::new_v4();
    add_account(
        &pool,
        uuid.to_string(),
        json.email.clone(),
        json.nickname.clone(),
        json.password.clone(),
    )
    .await
    .unwrap();
    let account = fetch_account_info(&pool, uuid).await.unwrap();
    web::Json(account)
}

#[post("/account/{user_id}/edit")]
async fn post_edit_account(
    context: web::Data<PgPool>,
    json: web::Json<EditAccountRequest>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = context;
    let uuid = uuid::Uuid::from_str(id.as_str()).unwrap();
    let token = get_token_from_request(&req).unwrap();
    let decode_id = decode_token(token).await.unwrap();
    if decode_id != uuid.to_string() {
        let response = PostAccountResponseAccount {
            account: get_empty_account_info().to_owned(),
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Unauthorized().json(response)
    } else {
        edit_account(
            &pool,
            uuid.to_string(),
            json.email.clone(),
            json.nickname.clone(),
        )
        .await
        .unwrap();
        let account = fetch_account_info(&pool, uuid).await.unwrap();
        let response = PostAccountResponseAccount {
            account: account.to_owned(),
            status: ResponseError::Success as u16,
        };
        HttpResponse::Ok().json(response)
    }
}

#[post("/account/{user_id}/delete")]
async fn post_delete_account(
    context: web::Data<PgPool>,
    id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let pool = context;
    let uuid = uuid::Uuid::from_str(id.as_str()).unwrap();
    let token = get_token_from_request(&req).unwrap();
    let decode_id = decode_token(token).await.unwrap();
    if decode_id != uuid.to_string() {
        let response = PostAccountResponseAccount {
            account: get_empty_account_info().to_owned(),
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Unauthorized().json(response)
    } else {
        delete_account(&pool, uuid.to_string()).await.unwrap();
        let response = PostAccountResponseAccount {
            account: get_empty_account_info().to_owned(),
            status: ResponseError::Success as u16,
        };
        HttpResponse::Ok().json(response)
    }
}

#[post("/login")]
async fn post_login_account(
    context: web::Data<PgPool>,
    json: web::Json<AccountAuthRequest>,
) -> impl Responder {
    let pool = context;
    if json.provider == AuthProvider::Plumage as u32 {
        let validate =
            validate_credentials_by_email(&pool, json.email.clone(), json.auth_key.clone()).await;
        if validate == true {
            let account = fetch_account_info_by_email(&pool, json.email.clone())
                .await
                .unwrap();
            let result = is_account_activated(&pool, account.id).await;
            match result {
                Ok(activated) => {
                    if activated {
                        let token = create_token(&account, json.provider).await.unwrap();
                        let response_json = AccountAuthResponse {
                            token: token.clone(),
                            status: ResponseError::Success as u16,
                        };
                        HttpResponse::Ok().json(response_json)
                    } else {
                        let response_json = AccountAuthResponse {
                            token: "".to_owned(),
                            status: ResponseError::Failure as u16,
                        };
                        HttpResponse::Ok().json(response_json)
                    }
                }
                Err(e) => {
                    let response_json = AccountAuthResponse {
                        token: "".to_owned(),
                        status: ResponseError::Success as u16,
                    };
                    HttpResponse::Ok().json(response_json)
                }
            }
        } else {
            let response_json = AccountAuthResponse {
                token: "".to_owned(),
                status: ResponseError::Success as u16,
            };
            HttpResponse::Unauthorized().json(response_json)
        }
    } else {
        let response_json = AccountAuthResponse {
            token: "".to_owned(),
            status: ResponseError::Success as u16,
        };
        HttpResponse::Unauthorized().json(response_json)
    }
}

pub fn get_token_from_request<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("x-csrf-token").unwrap().to_str().ok()
}

#[post("/send_verify_email_code")]
async fn send_verify_email_code(
    context: web::Data<PgPool>,
    json: web::Json<AccountSendVerifyEmailCodeRequest>,
) -> impl Responder {
    let pool = context;
    if json.email.is_empty() {
        let response_json = AccountSendVerifyEmailCodeResponse {
            code: 0,
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Ok().json(response_json)
    } else {
        let auth = TOTP::new(json.email.clone());
        let time1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let verify_code = auth.generate(60, time1);
        let response_json = AccountSendVerifyEmailCodeResponse {
            code: verify_code,
            status: ResponseError::Success as u16,
        };
        let account = fetch_account_info_by_email(&pool, json.email.clone())
            .await
            .unwrap();
        send_verify_email_link(account, verify_code).await.unwrap();
        HttpResponse::Ok().json(response_json)
    }
}

#[get("/account/{user_id}/verify_email/{code}")]
async fn verify_email(
    context: web::Data<PgPool>,
    info: web::Path<(String, u32)>,
) -> impl Responder {
    let pool = context;
    let info = info.into_inner();
    let uuid = uuid::Uuid::from_str(info.0.as_str()).unwrap();

    if info.1 == 0 {
        let response_json = AccountVerifyEmailCodeResponse {
            status: ResponseError::Failure as u16,
        };
        HttpResponse::Ok().json(response_json)
    } else {
        match fetch_account_info(&pool, uuid).await {
            Ok(account_info) => {
                let auth = TOTP::new(account_info.email.clone());
                let time1 = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let verify_code = auth.verify(info.1, 60, time1);
                if verify_code {
                    activated_account(&pool, uuid.to_string()).await.unwrap();
                    let response_json = AccountVerifyEmailCodeResponse {
                        status: ResponseError::Success as u16,
                    };
                    HttpResponse::Ok().json(response_json)
                } else {
                    let response_json = AccountVerifyEmailCodeResponse {
                        status: ResponseError::Failure as u16,
                    };
                    HttpResponse::Ok().json(response_json)
                }
            }
            Err(e) => {
                println!("{:?}", e);
                let response_json = AccountVerifyEmailCodeResponse {
                    status: ResponseError::Failure as u16,
                };
                HttpResponse::Ok().json(response_json)
            }
        }
    }
}

fn verify_email_content(nickname: String, link: String) -> String {
    let html = html! {
        p {
            "Hi "(nickname)
        }
        p {
            "Please confirm your email address to start receiving the rss feeds."
        }
        p class="btn-tims-green"{
            a href=(link){(link)}
        }
        p class="btn-tips" style="margin-bottom: 0;" {
            "If you didn't subscribe, please ignore this or reply this mail to RssMailer"
        }
        p class="btn-tips" {}
    };
    html.into_string()
}

async fn send_verify_email_link(account: AccountInfo, code: u32) -> Result<()> {
    let utc: DateTime<Utc> = Utc::now();
    let email_subject = utc.format("RSS mailer - verify account email").to_string();
    let to_address = account.email.to_string();
    let (smtp_username, smtp_password, smtp_server) = get_smtp_info(); 
    let link = format!(
        "https://127.0.0.1:8443/account/{}/verify_email/{}",
        account.id, code
    );
    let content = verify_email_content(account.nickname, link.clone());

    let email = lettre::Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject(email_subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("RssMailer from Plumage")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(content.to_string()),
                ),
        )
        .unwrap();
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("email send successfully");
            Ok(())
        }
        Err(_) => {
            println!("email send failed");
            Err(anyhow!("email send failed"))
        }
    }
}
