use super::*;
use crate::db::account::*;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

static K_ACCOUNT_EMAIL: &str = "mike@mail.com";
static K_ACCOUNT_NICKNAME: &str = "mike";
static K_ACCOUNT_PASSWORD: &str = "mike";

static K_ACCOUNT_EDIT_EMAIL: &str = "mike_1@mail.com";
static K_ACCOUNT_EDIT_NICKNAME: &str = "mike_1";

#[tokio::test]
async fn test_account() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(K_RSSMAILER_DB)
        .await?;
    let email = K_ACCOUNT_EMAIL;
    let nickname = K_ACCOUNT_NICKNAME;
    let password = K_ACCOUNT_PASSWORD;
    let edit_email = K_ACCOUNT_EDIT_EMAIL;
    let edit_nickname = K_ACCOUNT_NICKNAME;

    let new_id = Uuid::new_v4();
    add_account(
        &pool,
        new_id.to_string(),
        email.to_string(),
        nickname.to_string(),
        password.to_string(),
    )
    .await?;
    let account = fetch_account(&pool, new_id).await;
    let account = match account {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    assert_eq!(nickname, account.nickname);
    assert_eq!(email, account.email);
    assert!(!account.activated);

    let validate = validate_credentials(&pool, account.id, password.to_string()).await;
    assert!(validate);

    let account = fetch_account_info(&pool, account.id).await?;
    assert_eq!(nickname, account.nickname);
    assert_eq!(email, account.email);

    edit_account(
        &pool,
        account.id.to_string(),
        edit_email.to_string(),
        edit_nickname.to_string(),
    )
    .await?;
    let account = fetch_account(&pool, account.id).await;
    let account = match account {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    assert_eq!(edit_email, account.email);
    assert_eq!(edit_nickname, account.nickname);

    let account_id = account.id.to_string();

    activated_account(&pool, account.id.to_string()).await?;
    let account = fetch_account(&pool, account.id).await;
    let account = match account {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    assert!(account.activated);

    delete_account(&pool, account_id.to_string()).await?;
    let err = fetch_account(&pool, account.id).await.unwrap_err();
    println!("{}", err);
    Ok(())
}
