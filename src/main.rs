pub mod commands;
use crate::commands::ping::*;

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

use once_cell::sync::OnceCell;
use crate::commands::new_edition::*;

//global variable for mongodb client
static MONGOCLIENT: OnceCell<MongoClient> = OnceCell::new();

//handler discord, bot core
struct HandlerDiscord;
#[async_trait]
impl EventHandler for HandlerDiscord {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ping_setup(ctx.borrow()).await;
        new_edition_setup(ctx.borrow()).await
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction{
            Interaction::ApplicationCommand(command) => {
                unsafe {
                    match command.data.name.as_str() {
                        "ping" => ping_reactor(&command, &ctx).await,
                        "new_edition" => new_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                        _ => ()
                    }}},

            Interaction::ModalSubmit(mci) => {
                match mci.data.custom_id.as_str() {
                    "competition" => prompt_date_debut_inscription_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    _ => ()
                }},

            _ => (),
        }
    }
}

//main function, setup for clients
#[tokio::main]
async fn main() {
    //setup mongo client
    let uri = env::var("MONGODB_LOGIN").unwrap();
    let client = MongoClient::with_uri_str(&uri).await.unwrap();
    MONGOCLIENT.set(client).unwrap();

    //setup discord client
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