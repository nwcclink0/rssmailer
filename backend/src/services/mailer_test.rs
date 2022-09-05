use crate::services::mailer::*;
use anyhow::{anyhow, Result};

use crate::db::K_RSSMAILER_DB;
use crate::handlers::{
    account::*, rssfeed::post_rssfeeds, rssfeed::RssFeedAction, rssfeed::RssFeedRequest,
    rssfeed::RssFeedResponse, ResponseError,
};
use crate::{db::account::AccountInfo, handlers::AuthProvider};
use actix_cors::Cors;
use actix_test::TestServer;
use actix_web::{http::header, web::Data, App};
use sqlx::postgres::PgPoolOptions;

use chrono::prelude::*;
use chrono::Duration as chrono_duration;
use tokio::{time::sleep, time::Duration};

pub static K_ACCOUNT_EMAIL: &str = dotenv!("TEST_ACCOUNT");
pub static K_ACCOUNT_NICKNAME: &str = dotenv!("TEST_NICKNAME");
pub static K_ACCOUNT_PASSWORD: &str = dotenv!("TEST_PASSWORD");

pub static K_RSS_1: &str = "https://gnn.gamer.com.tw/rss.xml";
pub static K_RSS_2: &str = "https://news.gamme.com.tw/category/hotchick/feed";

async fn server_up() -> TestServer {
    let db_url = K_RSSMAILER_DB;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();
    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();
    let srv = actix_test::start(move || {
        let cros = Cors::default()
            // .allowed_origin("127.0.0.1")
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                csrf_token_header.clone(),
            ])
            .expose_headers(vec![csrf_token_header.clone()])
            .max_age(3600);

        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cros)
            .service(post_add_account)
            .service(post_edit_account)
            .service(post_delete_account)
            .service(post_account)
            .service(post_login_account)
            .service(send_verify_email_code)
            .service(verify_email)
            .service(post_rssfeeds)
    });
    srv
}

async fn account_init() -> (String, String) {
    let srv = server_up().await;
    let add_account_email = K_ACCOUNT_EMAIL;
    let add_account_nickname = K_ACCOUNT_NICKNAME;
    let add_account_password = K_ACCOUNT_PASSWORD;
    let add_account_path = "/account/add".to_owned();
    let mut response = srv
        .post(add_account_path.clone())
        .send_json(&AddAccountRequest {
            email: add_account_email.to_owned(),
            nickname: add_account_nickname.to_owned(),
            password: add_account_password.to_owned(),
        })
        .await
        .unwrap();
    assert!(response.status().is_success());
    let add_account = response.json::<AccountInfo>().await.unwrap();

    let query_path = "/account/send_verify_email_code";
    let mut response = srv
        .post(query_path.clone())
        .send_json(&AccountSendVerifyEmailCodeRequest {
            email: add_account_email.to_owned(),
        })
        .await
        .unwrap();
    assert!(response.status().is_success());
    let code_response = response
        .json::<AccountSendVerifyEmailCodeResponse>()
        .await
        .unwrap();
    assert_ne!(code_response.code, 0);

    let query_path = format!(
        "/account/{}/verify_email/{}",
        add_account.id, code_response.code
    );
    let mut response = srv.get(query_path.clone()).send().await.unwrap();
    assert!(response.status().is_success());
    let verify_response = response
        .json::<AccountVerifyEmailCodeResponse>()
        .await
        .unwrap();
    assert_eq!(verify_response.status, ResponseError::Success as u16);

    let query_path = format!("/login");
    let mut response = srv
        .post(query_path.clone())
        .send_json(&AccountAuthRequest {
            email: add_account.email,
            auth_key: add_account_password.to_owned(),
            provider: AuthProvider::Plumage as u32,
        })
        .await
        .unwrap();
    assert!(response.status().is_success());
    let json_response = response.json::<AccountAuthResponse>().await.unwrap();
    let token = json_response.token;
    assert_ne!(token.clone().len(), 0);

    (add_account.id.to_string(), token)
}

async fn rsslink_init(id: String, token: String) {
    let srv = server_up().await;
    let query_path = format!("/rssfeed/{}", id);
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send_json(&RssFeedRequest {
            action: RssFeedAction::Add.as_ref().to_owned(),
            link: K_RSS_1.to_owned(),
            token: token.to_owned(),
        })
        .await
        .unwrap();
    let json_response = response.json::<RssFeedResponse>().await.unwrap();
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send_json(&RssFeedRequest {
            action: RssFeedAction::Add.as_ref().to_owned(),
            link: K_RSS_2.to_owned(),
            token: token.to_owned(),
        })
        .await
        .unwrap();
    let json_response = response.json::<RssFeedResponse>().await.unwrap();
}

async fn account_uninit(id: String, token: String) {
    let srv = server_up().await;
    let query_path = format!("/account/{}/delete", id);
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let json_response = response.json::<PostAccountResponseAccount>().await.unwrap();
    assert_eq!(json_response.status, ResponseError::Success as u16);
}

async fn rsslink_uninit(id: String, token: String) {
    let srv = server_up().await;
    let query_path = format!("/rssfeed/{}", id);
    let rss_list = vec![K_RSS_1, K_RSS_2];
    for rss in rss_list {
        let mut response = srv
            .post(query_path.clone())
            .insert_header(("x-csrf-token", token.clone()))
            .send_json(&RssFeedRequest {
                action: RssFeedAction::Delete.as_ref().to_owned(),
                link: rss.to_owned(),
                token: token.to_owned(),
            })
            .await
            .unwrap();
        assert!(response.status().is_success());
        let json_response = response.json::<RssFeedResponse>().await.unwrap();
        assert_eq!(json_response.status, ResponseError::Success as u16);
    }
}

#[actix_rt::test]
async fn test_mailer() -> Result<()> {
    let (id, token) = account_init().await;
    rsslink_init(id.clone(), token.clone()).await;
    let result = Mailer::new().await;
    match result {
        Ok(mailer) => {
            mailer.run().await;
            account_uninit(id.clone(), token.clone()).await;
            rsslink_uninit(id.clone(), token.clone()).await;
            Ok(())
        }
        Err(e) => {
            account_uninit(id.clone(), token.clone()).await;
            rsslink_uninit(id.clone(), token.clone()).await;
            Err(anyhow!("{:?}", e))
        }
    }
}

#[actix_rt::test]
async fn test_mailer_block_on() -> Result<()> {
    let mut pre = Local::now();
    let (id, token) = account_init().await;
    rsslink_init(id.clone(), token.clone()).await;
    let mailer = Mailer::new().await.unwrap();
    loop {
        let mailer = mailer.clone();
        let local = Local::now();
        let local_hour = local.hour();
        let timediff = local - pre;
        if timediff > chrono_duration::seconds(15) {
            println!("hit 15 second local: {:?}, pre: {:?}", local, pre);
            if local_hour == pre.hour() {
                tokio::spawn(async {
                    println!("send mail");
                    mailer.send().await.unwrap();
                })
                .await
                .unwrap();
                break;
            }
            pre = local;
        }
        sleep(Duration::from_secs(1)).await;
        println!("1 second pass");
    }
    account_uninit(id.clone(), token.clone()).await;
    rsslink_uninit(id.clone(), token.clone()).await;

    Ok(())
}
