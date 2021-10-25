use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::{Executor, PgPool};

static K_TABLE_NAME: &str = "rssfeed";

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct RssFeed {
    pub account_id: uuid::Uuid,
    pub link: String,
    pub add_date: OffsetDateTime,
}

pub async fn fetch_rssfeeds(pool: &PgPool) -> Result<Vec<RssFeed>, sqlx::Error> {
    let rows = sqlx::query_as!(RssFeed, "SELECT account_id, link, add_date from rssfeed")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn add_rssfeed(
    pool: &PgPool,
    account_id: uuid::Uuid,
    link: String,
) -> Result<(), sqlx::Error> {
    let command: String = format!(
        "insert into {} values ('{}', '{}');",
        K_TABLE_NAME, account_id, link
    );
    println!("sql command: {}", command);
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn delete_rssfeed(
    pool: &PgPool,
    account_id: uuid::Uuid,
    link: String,
) -> Result<(), sqlx::Error> {
    let command: String = format!(
        "delete from {} where account_id = '{}' and link = '{}';",
        K_TABLE_NAME, account_id, link
    );
    pool.execute(command.as_str()).await?;
    Ok(())
}

pub async fn fetch_account_rssfeeds(
    pool: &PgPool,
    account_id: uuid::Uuid,
) -> Result<Vec<RssFeed>, sqlx::Error> {
    let command = format!("select * from rssfeed where account_id = '{}'", account_id);
    let rows = sqlx::query_as(command.as_str()).fetch_all(pool).await?;
    Ok(rows)
}
