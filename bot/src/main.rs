mod commands;
mod container;
mod handler;
use commands::{meta::*, owner::*, quote::*};
use container::ShardManagerContainer;
use handler::Handler;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};
use std::{collections::HashSet, env, panic};

#[group]
#[commands(quote, ping, fifty_mm, thirty_five_mm)]
struct General;

#[tokio::main]
async fn main() {
    // load config
    dotenv::from_filename("./.env").expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // initialize log output
    tracing_subscriber::fmt::init();

    // load owners
    let http = Http::new(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // initialize framework for commands
    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(true).prefix("!").owners(owners))
        .group(&GENERAL_GROUP);

    // set intents and build client
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    // spawn thread
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // finally, connect and start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
