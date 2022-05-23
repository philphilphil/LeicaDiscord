use rusqlite::{Connection, Result};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("q")]
async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
    match get_quote() {
        Ok(quote) => {
            if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                println!("Error sending message: {:?}", why);
            }
        }
        Err(quote) => {
            println!("Issue getting data from db: {:?}", quote);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct KenRQuote {
    id: i32,
    quote: String,
}

fn get_quote() -> Result<String, rusqlite::Error> {
    let path = "db/app.db";
    let conn = Connection::open(path)?;

    check_and_reset_quote_states(&conn)?;

    if let Ok(quote) = get_random_quote(&conn) {
        let mut query = conn.prepare("UPDATE KenRQuotes SET LastPosted = DATE() WHERE id = (?)")?;
        query.execute([quote.id])?;
        Ok(quote.quote)
    } else {
        Err(rusqlite::Error::InvalidQuery)
    }
}

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
