use super::*;
use crate::db::rssfeed::add_rssfeed;
use crate::db::rssfeed::delete_rssfeed;
use crate::db::rssfeed::fetch_account_rssfeeds;
use crate::db::rssfeed::fetch_rssfeeds;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

static K_RSS_LINK: &str = "https://gnn.gamer.com.tw/rss.xml";

#[tokio::test]
async fn test_rssfeed() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(K_RSSMAILER_DB)
        .await?;
    let account_id = Uuid::new_v4();
    let rsslink = K_RSS_LINK;
    add_rssfeed(&pool, account_id, rsslink.to_string()).await?;
    let feeds = fetch_account_rssfeeds(&pool, account_id).await?;
    for feed in feeds {
        assert_eq!(account_id, feed.account_id);
        assert_eq!(rsslink, feed.link);
    }
    delete_rssfeed(&pool, account_id, rsslink.to_string()).await?;
    let feeds = fetch_rssfeeds(&pool).await?;
    assert_eq!(feeds.len(), 0);
    Ok(())
}
