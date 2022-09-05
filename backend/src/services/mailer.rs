use super::K_CHECK_RSSFEED_TIME;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration as chrono_duration, Utc};
use dotenv::dotenv;
use lettre::transport::smtp;
use maud::html;
use rss::Channel;
use sqlx::PgPool;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{header, MultiPart, SinglePart},
    SmtpTransport, Transport,
};

use crate::db::account::{fetch_accounts};
use crate::db::rssfeed::fetch_account_rssfeeds;
use crate::db::K_RSSMAILER_DB;
use chrono::prelude::*;
use ctrlc;
use sqlx::postgres::PgPoolOptions;
use std::sync::mpsc::channel;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct Mailer {
    pool: PgPool,
}

pub fn get_smtp_info() -> (&'static str, &'static str, &'static str) {
    let account = dotenv!("SMTP_ACCOUNT");
    let password = dotenv!("SMTP_PASSWORD");
    let smtp_server = dotenv!("SMTP_SERVER");
    return (account, password, smtp_server)
}

impl Mailer {
    pub async fn new() -> Result<Self> {
        let db_url = K_RSSMAILER_DB;
        let result = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await;
        match result {
            Ok(pool) => Ok(Mailer { pool }),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }

    pub async fn send(self: Self) -> Result<()> {
        let result = fetch_accounts(&self.pool).await;
        match result {
            Ok(accounts) => {
                for account in accounts {
                    let rssfeeds = fetch_account_rssfeeds(&self.pool, account.id)
                        .await
                        .unwrap();
                    let mut links = Vec::new();
                    for rssfeed in rssfeeds {
                        let link = rssfeed.link;
                        links.push(link)
                    }
                    let task  = tokio::task::spawn_blocking(|| async {
                        send_mail(account.email, links).await.unwrap();
                    });
                task.await.unwrap().await;
                }
                Ok(())
            }
            Err(e) => Err(anyhow!("error: {:?}", e)),
        }
    }

    pub async fn run(self: Self) {
        let (tx, rx) = channel();
        ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");
        let mut pre = Local::now();
        loop {
            let moved_self = self.clone();
            let local = Local::now();
            let local_hour = local.hour();
            let timediff = local - pre;
            if timediff > chrono_duration::seconds(3600 * 24) {
                if local_hour == K_CHECK_RSSFEED_TIME {
                    println!("send mail");
                    moved_self.send().await.unwrap();
                    pre = Local::now();
                }
                pre = local;
            }
            let recv_result = rx.recv_timeout(Duration::from_secs(1));
            println!("wait for timeout or terminated");
            match recv_result {
                Ok(recv) => {
                    if recv {
                        break;
                    }
                }
                Err(_) => {}
            }
        }
    }
}

struct Content {
    number: u32,
    provider: String,
    today_summary: Vec<ContentBody>,
}

struct ContentBody {
    title: String,
    summary: String,
    pub_date: String,
    link: String,
}

async fn get_feed(link: String) -> Result<Channel> {
    let content = reqwest::get(link).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn email_title(issue_number: i32, contents: &Vec<Content>) -> String {
    // fn email_title(issue_number: i32) -> &str {
    let issue_number: String = issue_number.to_string();
    let html = html!(
        div class="bg-gray-200 px-4 py-6 mb-5 text-gray-500" style="box-sizing: border-box; --bg-opacity: 1; background-color: rgba(237,242,247,var(--bg-opacity)); margin-bottom: 1.25rem; --text-opacity: 1; color: rgba(160,174,192,var(--text-opacity)); padding: 1.5rem 1rem; border: 0 solid #e2e8f0;"{
                    "RSS mailer - Issue #"(issue_number.to_string())
                div class="border-t-2 my-3" style="box-sizing: border-box; margin-top: .75rem; margin-bottom: .75rem; border-color: #e2e8f0; border-style: solid; border-width: 2px 0 0;"{}
                div class="text-sm" style="box-sizing: border-box; font-size: .875rem; border: 0 solid #e2e8f0;" {
                    div class="text-gray-700 mb-1" style="box-sizing: border-box; margin-bottom: .25rem; --text-opacity: 1; color: rgba(74,85,104,var(--text-opacity)); border: 0 solid #e2e8f0;"{
                    "In today's issue:"
                        @for content in contents {
                            div class="text-gray-700" style="box-sizing: border-box; --text-opacity: 1; color: rgba(74,85,104,var(--text-opacity)); border: 0 solid #e2e8f0;"{
                                               "- "(content.number.to_string().to_owned()) " from "(content.provider)
                            }
                        }
                    }
                }
        }
    );
    html.into_string()
}

fn email_content(content: Content) -> String {
    let html = html!(
        div class="mb-3 mt-6 text-gray-600" style="box-sizing: border-box; margin-bottom: .5rem; margin-top: 1.5rem; --text-opacity: 1; color: rgba(113,128,150,var(--text-opacity)); border: 0 solid #e2e8f0;" {
            (content.provider)
        }
        div class="mb-9 border bg-white border-gray-300 rounded-lg p-4 pt-3" style="box-sizing: border-box; --bg-opacity: 1; background-color: rgba(255,255,255,var(--bg-opacity)); --border-opacity: 1; border-radius: .5rem; margin-bottom: 2rem; padding: .75rem 1rem 1rem; border: 1px solid;"{
        @for summary in content.today_summary {
            div class="text-base font-normal text-black font-medium" style="box-sizing: border-box; font-weight: 499; font-size: 1rem; --text-opacity: 1; color: rgba(0,0,0,var(--text-opacity)); border: 0 solid #e2e8f0;" {
            a class="black underline" href={(summary.link)} style="background-color: transparent; box-sizing: border-box; color: inherit; text-decoration: underline; border: -1 solid #e2e8f0;"{
                (summary.title)
            }
            div class="text-xs text-gray-601 font-normal mb-2" style="box-sizing: border-box; font-weight: 400; font-size: .75rem; margin-bottom: .5rem; --text-opacity: 1; color: rgba(113,128,150,var(--text-opacity)); border: 0 solid #e2e8f0;"{
                (content.provider)" - "(summary.pub_date)" - "
            a class="text-red-401 underline" target="_blank" href={"https://getpocket.com/edit.php?url="(summary.link)} style="background-color: transparent; box-sizing: border-box; color: rgba(252,129,129,var(--text-opacity)); text-decoration: underline; --text-opacity: 1; border: 0 solid #e2e8f0;"{"Save to Pocket"}}

            div class="text-base text-left feed-content" style="box-sizing: border-box; font-size: 1rem; border: 0 solid #e2e8f0;" align="left"{
               (summary.summary)
            }
            div class="bg-gray-300 h-px w-full mt-6 mb-6" style="box-sizing: border-box; --bg-opacity: 1; background-color: rgba(226,232,240,var(--bg-opacity)); height: 1px; margin-top: 1.5rem; margin-bottom: 1.5rem; width: 100%; border: 0 solid #e2e8f0;"{}
        }}
    }

    );
    html.into_string()
}

async fn send_mail(email: String, links: Vec<String>) -> Result<()> {
    let mut contents = Vec::new();
    for link in links {
        let response = get_feed(link.to_owned()).await;
        match response {
            Ok(channel) => {
                let mut content = Content {
                    number: 0,
                    provider: channel.title.clone(),
                    today_summary: Vec::new(),
                };
                for item in &channel.items {
                    content.number = content.number + 1;
                    let summary = ContentBody {
                        title: item.title.clone().unwrap(),
                        pub_date: item.pub_date.clone().unwrap(),
                        summary: item.description.clone().unwrap(),
                        link: item.link.clone().unwrap(),
                    };
                    content.today_summary.push(summary);
                }
                contents.push(content);
            }
            Err(e) => {
                println!("{:}", e);
            }
        }
    }

    let email_body = email_title(0, &contents);
    let mut all_email_content: String = "".to_owned();
    for new_content in contents {
        let email_content = email_content(new_content);
        all_email_content.push_str(email_content.as_str());
    }

    let utc: DateTime<Utc> = Utc::now();
    let email_subject = utc.format("RSS mailer - %Y-%m-%d").to_string();

    let to_address = email.to_string();
    let (smtp_username, smtp_password, smtp_server) = get_smtp_info();
    // let email_subject = "rss-mailer - ".to_string().push_str(today.as_str());
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
                        .body(email_body.to_string()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(all_email_content.to_string()),
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

