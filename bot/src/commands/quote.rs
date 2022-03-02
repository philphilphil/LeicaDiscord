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

    let mut quotes = get_random_quote(&conn)?;

    if quotes.len() == 0 {
        // no quote without LastPosted-Date, resetting
        conn.execute("UPDATE KenRQuotes SET LastPosted = NULL", [])?;
        quotes = get_random_quote(&conn)?;
    } else {
        let mut stmt = conn.prepare("UPDATE KenRQuotes SET LastPosted = DATE() WHERE id = (?)")?;
        stmt.execute([quotes[0].id])?;
    }

    Ok(quotes[0].quote.clone())
}

fn get_random_quote(conn: &Connection) -> Result<Vec<KenRQuote>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, quote FROM KenRQuotes WHERE LastPosted IS NULL ORDER BY RANDOM() LIMIT 1;",
    )?;

    let quote = stmt
        .query_map([], |row| {
            Ok(KenRQuote {
                id: row.get(0)?,
                quote: row.get(1)?,
            })
        })?
        .map(|x| x.unwrap())
        .collect();

    Ok(quote)
}
