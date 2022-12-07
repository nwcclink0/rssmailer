use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

static K_TABLE_NAME: &str = "account";

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Account {
    pub id: uuid::Uuid,
    pub email: String,
    pub nickname: String,
    pub activated: bool,
    pub password_hash: String,
    pub add_date: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: uuid::Uuid,
    pub email: String,
    pub nickname: String,
}

pub async fn fetch_accounts(pool: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Account,
        "SELECT id, email, nickname, activated, add_date, password_hash from account"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

fn account_to_account_info(account: Account) -> AccountInfo {
    AccountInfo {
        id: account.id.clone(),
        email: account.email.clone(),
        nickname: account.nickname.clone(),
    }
}

pub async fn add_account(
    pool: &PgPool,
    id: String,
    email: String,
    nickname: String,
    password: String,
) -> Result<(), sqlx::Error> {
    let password_hash = generate_password_hash(password).unwrap();
    let command: String = format!(
        "insert into {} (id, email, nickname, activated, password_hash) values ('{}', '{}', '{}', '{}', '{}');",
        K_TABLE_NAME, id, email, nickname, false, password_hash
    );
    println!("sql command: {}", command);
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn activated_account(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
    let command: String = format!(
        "update {} set activated = '{}' where id = '{}';",
        K_TABLE_NAME, true, id
    );
    println!("sql command: {}", command);
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn edit_account(
    pool: &PgPool,
    id: String,
    email: String,
    nickname: String,
) -> Result<(), sqlx::Error> {
    let command: String = format!(
        "update {} set email= '{}', nickname = '{}' where id = '{}';",
        K_TABLE_NAME, email, nickname, id
    );
    println!("sql command: {}", command);
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn delete_account(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
    let command: String = format!("delete from {} where id = '{}';", K_TABLE_NAME, id);
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn fetch_account(pool: &PgPool, id: uuid::Uuid) -> Result<Account, sqlx::Error> {
    let command = format!("select * from account where id = '{}'", id);
    let row: Result<Account, sqlx::Error> = sqlx::query_as(command.as_str()).fetch_one(pool).await;
    match row {
        Ok(account) => Ok(account),
        Err(e) => Err(e),
    }
}

async fn fetch_account_by_email(pool: &PgPool, email: String) -> Result<Account, sqlx::Error> {
    let command = format!("select * from account where email = '{}'", email);
    let row: Result<Account, sqlx::Error> = sqlx::query_as(command.as_str()).fetch_one(pool).await;
    match row {
        Ok(account) => Ok(account),
        Err(e) => Err(e),
    }
}

pub async fn fetch_account_info(pool: &PgPool, id: uuid::Uuid) -> Result<AccountInfo, sqlx::Error> {
    let result = fetch_account(pool, id).await;
    match result {
        Ok(account) => {
            let account_info = account_to_account_info(account);
            Ok(account_info)
        }
        Err(e) => Err(e),
    }
}

pub async fn fetch_account_info_by_email(
    pool: &PgPool,
    email: String,
) -> Result<AccountInfo, sqlx::Error> {
    let result = fetch_account_by_email(pool, email).await;
    match result {
        Ok(account) => {
            let account_info = account_to_account_info(account);
            Ok(account_info)
        }
        Err(e) => Err(e),
    }
}

pub async fn is_account_activated(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let result = fetch_account(pool, id).await;
    match result {
        Ok(account) => {
            if account.activated {
                Ok(true)
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_empty_account_info() -> AccountInfo {
    let uuid = Uuid::parse_str("00000000000000000000000000000000").unwrap();
    AccountInfo {
        id: uuid,
        email: "".to_owned(),
        nickname: "".to_owned(),
    }
}

fn generate_password_hash(password: String) -> Result<String, sqlx::Error> {
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let argon2 = Argon2::default();
    // let password_hash = argon2
    //     .hash_password(password.as_bytes(), &salt)
    //     .unwrap()
    //     .to_string();
    let result = argon2.hash_password(password.as_bytes(), &salt);
    match result {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => {
            println!("{:?}", e);
            Err(sqlx::Error::RowNotFound)
        }
    }
}

pub async fn validate_credentials(pool: &PgPool, id: uuid::Uuid, password: String) -> bool {
    let account = fetch_account(&pool, id).await.unwrap();
    let expected_password_hash = PasswordHash::new(&account.password_hash).unwrap();
    let result = Argon2::default().verify_password(password.as_bytes(), &expected_password_hash);
    match result {
        Ok(()) => true,
        Error => false,
    }
}

pub async fn validate_credentials_by_email(pool: &PgPool, email: String, password: String) -> bool {
    let account = fetch_account_by_email(&pool, email).await.unwrap();
    let expected_password_hash = PasswordHash::new(&account.password_hash).unwrap();
    let result = Argon2::default().verify_password(password.as_bytes(), &expected_password_hash);
    match result {
        Ok(()) => true,
        Error => false,
    }
}
