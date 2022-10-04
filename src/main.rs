extern crate core;

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
use crate::commands::common_functions::reset_global_application_command;
use crate::commands::new_edition::*;
use crate::commands::delete_edition::*;
use crate::commands::edit_edition::*;

//global variable for mongodb client
static MONGOCLIENT: OnceCell<MongoClient> = OnceCell::new();

//handler discord, bot core
struct HandlerDiscord;
#[async_trait]
impl EventHandler for HandlerDiscord {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ping_setup(ctx.borrow()).await;
        new_edition_setup(ctx.borrow()).await;
        delete_edition_setup(ctx.borrow()).await;
        edit_edition_setup(ctx.borrow()).await;
        println!("{} is connected!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction{
            Interaction::ApplicationCommand(command) => {
                unsafe {
                    match command.data.name.as_str() {
                        "ping" => ping_reactor(&command, &ctx).await,
                        "new_edition" => new_edition_reactor(&command, &ctx).await,
                        "delete_edition" => delete_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                        "edit_edition" => edit_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                        _ => ()
                    }}},

            Interaction::ModalSubmit(mci) => {
                match mci.data.custom_id.as_str() {
                    "new_edition_modal" => prompt_edition_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    //"edit_start_inscriptions_end" => edit_start_inscriptions_end(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    _ => ()
                }},

            Interaction::MessageComponent(mci) => {
                match mci.data.custom_id.as_str() {
                    "delete_edition_modal" => delete_edition_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    "edit_start_inscriptions" => edit_start_inscriptions(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    "edit_end_inscriptions" => println!("ok : edit_end_inscriptions"),
                    "edit_start_competition" => println!("ok : edit_start_competition"),
                    "edit_end_competition" => println!("ok : edit_end_competition"),
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