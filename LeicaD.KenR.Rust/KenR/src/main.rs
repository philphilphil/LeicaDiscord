use core::fmt::Error;
use rusqlite::{Connection, Result};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[derive(Debug)]
struct KenRQuote {
    id: i32,
    quote: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!q" {
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
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn get_quote() -> Result<String, rusqlite::Error> {
    let path = "db/app.db";
    let conn = Connection::open(path).unwrap();

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

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
