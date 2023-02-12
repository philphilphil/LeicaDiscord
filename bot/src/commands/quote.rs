use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::TimeZone;
use rusqlite::{Connection, Result};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use tracing::error;

enum QuoteResult {
    OnCooldown,
    Quote(String),
}

#[derive(Debug)]
struct KenRQuote {
    id: i32,
    quote: String,
}

const COOLDOWN_MESSAGE: &str =
    "Due to increased cost of my growing family I can only post a quote every 20 seconds.";

async fn post_quote(ctx: &Context, msg: &Message) -> CommandResult {
    match get_quote() {
        Ok(quote) => match quote {
            QuoteResult::OnCooldown => {
                if let Err(why) = msg
                    .author
                    .dm(&ctx.http, |m| m.content(COOLDOWN_MESSAGE))
                    .await
                {
                    error!("Error sending message: {:?}", why);
                }
                msg.delete(&ctx.http).await.unwrap();
            }
            QuoteResult::Quote(q) => {
                if let Err(why) = msg.channel_id.say(&ctx.http, q).await {
                    error!("Error sending message: {:?}", why);
                }
            }
        },
        Err(why) => {
            error!("Issue getting data from db: {:?}", why);
        }
    }

    Ok(())
}

#[command]
#[aliases("q")]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    post_quote(ctx, msg).await
}

// Say quote but delete trigger message (hidden feature ;))
#[command]
#[aliases("qs")]
async fn quotesilent(ctx: &Context, msg: &Message) -> CommandResult {
    post_quote(ctx, msg).await?;
    _ = msg.delete(&ctx.http).await;
    Ok(())
}

fn get_quote() -> Result<QuoteResult, rusqlite::Error> {
    let path = "../db/app.db";
    let conn = Connection::open(path)?;

    if on_cooldown(&conn)? {
        return Ok(QuoteResult::OnCooldown);
    }

    check_and_reset_quote_states(&conn)?;

    if let Ok(quote) = get_random_quote(&conn) {
        let mut query = conn.prepare(
            "UPDATE KenRQuotes SET LastPosted = datetime('now', 'localtime') WHERE id = (?)",
        )?;
        query.execute([quote.id])?;
        Ok(QuoteResult::Quote(quote.quote))
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}

/// If the duration since the last posted quote is less then 20 seconds,
/// the command is on cooldown
fn on_cooldown(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let last_post: chrono::NaiveDateTime = conn.query_row(
        "SELECT LastPosted FROM KenRQuotes ORDER BY LastPosted DESC LIMIT 1",
        [],
        |r| r.get(0),
    )?;

    let last_post: DateTime<Local> = Local.from_local_datetime(&last_post).unwrap();

    if last_post + Duration::seconds(20) > Local::now() {
        return Ok(true);
    }

    Ok(false)
}

/// If no more quotes without a LastPosted date are available,
/// reset all to NULL and restart the cycle
fn check_and_reset_quote_states(conn: &Connection) -> Result<(), rusqlite::Error> {
    let available_quote_count: u64 = conn.query_row(
        "SELECT COUNT(*) FROM KenRQuotes WHERE LastPosted IS NULL",
        [],
        |r| r.get(0),
    )?;

    if available_quote_count == 0 {
        conn.execute("UPDATE KenRQuotes SET LastPosted = NULL", [])?;
    }

    Ok(())
}

/// Get a random quote that was not yet posted in this cycle
fn get_random_quote(conn: &Connection) -> Result<KenRQuote, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT id, quote FROM KenRQuotes WHERE LastPosted IS NULL ORDER BY RANDOM() LIMIT 1;",
    )?;

    let mut quote = query.query_map([], |row| {
        Ok(KenRQuote {
            id: row.get(0)?,
            quote: row.get(1)?,
        })
    })?;

    // its ugly to get just one line but i cant find a better way right now..
    Ok(quote.next().unwrap().unwrap())
}
