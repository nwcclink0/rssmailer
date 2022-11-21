use super::ResponseError;
use crate::db::K_RSSMAILER_DB;
use crate::handlers::account::*;
use crate::{db::account::AccountInfo, handlers::AuthProvider};
use actix_cors::Cors;
use actix_web::{http::header, web::Data, App};
use sqlx::postgres::PgPoolOptions;

#[actix_rt::test]
async fn test_account_post() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = K_RSSMAILER_DB;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
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
    });
    let add_account_email = dotenv!("TEST_ACCOUNT").clone();
    let add_account_nickname = "fakennickame".clone();
    let add_account_password = "fakepassword".clone();
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

    let query_path = "/send_verify_email_code";
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

    let query_path = format!("/account/{}", add_account.id);
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let json_response = response.json::<PostAccountResponseAccount>().await.unwrap();

    let account_id = json_response.account.id;
    assert_eq!(account_id, add_account.id.clone());

    let account_nickname = json_response.account.nickname.clone();
    assert_eq!(account_nickname, add_account_nickname.to_owned());

    let account_email = json_response.account.email.clone();
    assert_eq!(account_email, add_account_email.to_owned());

    let query_path = format!("/account/{}/edit", add_account.id);
    let edit_email = "another_email@mail.com";
    let edit_nickname = "another_nickname";
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send_json(&EditAccountRequest {
            email: edit_email.to_owned(),
            nickname: edit_nickname.to_owned(),
        })
        .await
        .unwrap();
    assert!(response.status().is_success());
    let json_response = response.json::<PostAccountResponseAccount>().await.unwrap();

    let account_id = json_response.account.id;
    assert_eq!(account_id, add_account.id.clone());

    let account_nickname = json_response.account.nickname.clone();
    assert_eq!(account_nickname, edit_nickname.clone());

    let account_email = json_response.account.email.clone();
    assert_eq!(account_email, edit_email.to_owned());

    let query_path = format!("/account/{}/delete", add_account.id);
    let mut response = srv
        .post(query_path.clone())
        .insert_header(("x-csrf-token", token.clone()))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    let json_response = response.json::<PostAccountResponseAccount>().await.unwrap();
    assert_eq!(json_response.status, ResponseError::Success as u16);

    Ok(())
}
