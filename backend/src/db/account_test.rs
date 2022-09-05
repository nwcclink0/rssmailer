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
    let mut account_id = "".to_owned();

    let new_id = Uuid::new_v4();
    add_account(
        &pool,
        new_id.to_string(),
        email.to_string(),
        nickname.to_string(),
        password.to_string(),
    )
    .await?;
    let accounts = fetch_accounts(&pool).await?;
    assert_eq!(accounts.len(), 1);
    assert_eq!(nickname, accounts[0].nickname);
    assert_eq!(email, accounts[0].email);
    assert_eq!(false, accounts[0].activated);

    let validate = validate_credentials(&pool, accounts[0].id, password.to_string()).await;
    assert_eq!(validate, true);

    let account = fetch_account_info(&pool, accounts[0].id).await?;
    assert_eq!(nickname, account.nickname);
    assert_eq!(email, account.email);

    edit_account(
        &pool,
        accounts[0].id.to_string(),
        edit_email.to_string(),
        edit_nickname.to_string(),
    )
    .await?;
    let accounts = fetch_accounts(&pool).await?;
    assert_eq!(accounts.len(), 1);
    assert_eq!(edit_email, accounts[0].email);
    assert_eq!(edit_nickname, accounts[0].nickname);

    account_id = accounts[0].id.to_string().clone();

    let accounts = fetch_accounts(&pool).await?;
    assert_eq!(accounts.len(), 1);
    activated_account(&pool, accounts[0].id.to_string()).await?;
    let accounts = fetch_accounts(&pool).await?;
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].activated, true);

    delete_account(&pool, account_id.to_string()).await?;
    let accounts = fetch_accounts(&pool).await?;
    assert_eq!(accounts.len(), 0);
    Ok(())
}
