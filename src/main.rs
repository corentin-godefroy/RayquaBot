//Bot discord en rust

pub mod command_reactor;
pub mod mongo_functions;
pub mod command_setup;

use std::borrow::Borrow;

use serenity::{
    async_trait,
    model::{gateway::Ready},
    prelude::*,
};
use std::env;
use mongodb::bson::doc;
use serenity::framework::StandardFramework;
use serenity::model::application::interaction::{Interaction};

use mongodb::{Client as MongoClient};
use crate::mongo_functions::mongo_functions::new_edition_insertion;

use once_cell::sync::OnceCell;
use crate::command_reactor::command_reactor::{new_edition_reactor, ping_reactor};
use crate::command_setup::command_setup::{new_edition_setup, ping_setup};

static CLIENT: OnceCell<MongoClient> = OnceCell::new();

struct HandlerDiscord;
#[async_trait]
impl EventHandler for HandlerDiscord {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ping_setup(ctx.borrow()).await;
        new_edition_setup(ctx.borrow()).await
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            unsafe {
                match command.data.name.as_str() {
                    "ping" => ping_reactor(command, ctx).await,
                    "new_edition" => new_edition_reactor(CLIENT.get().unwrap() , command, ctx).await,
                    _ => (),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let uri = env::var("MONGODB_LOGIN").unwrap();
    let client = MongoClient::with_uri_str(&uri).await.unwrap();
    CLIENT.set(client).unwrap();

    let token = env::var("DISCORD_TOKEN").expect("token");

    let mut client = Client::builder(&token, Default::default())
        .event_handler(HandlerDiscord)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
