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
#[commands(quote, ping, quit)]
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

    // Fetch owners and id
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // create framework for bot commands
    let framework = StandardFramework::new().group(&GENERAL_GROUP);

    // build clients, register framework and event handler
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // initialize shared manager container
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
