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
use crate::commands::new_edition::*;
use crate::commands::delete_edition::*;
use crate::commands::edit_edition::*;
use crate::commands::constants::*;
use crate::commands::get_edition::{get_edition_end, get_edition_reactor, get_edition_setup};
use crate::TypeDate::{EndCompetition, EndRegistration, StartCompetition, StartRegistration};

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
        get_edition_setup(ctx.borrow()).await;
        println!("{} is connected!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction{
            Interaction::ApplicationCommand(command) => {

                match command.data.name.as_str() {
                    PING => ping_reactor(&command, &ctx).await,
                    NEW_EDITION => new_edition(&command, &ctx).await,
                    DELETE_EDITION => delete_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    EDIT_EDITION => edit_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    GET_EDITION => get_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    _ => ()
                }},

            Interaction::ModalSubmit(mci) => {
                match mci.data.custom_id.as_str() {
                    CREATE_NEW_EDITION => new_edition_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    //EDIT_START_EDITION_END => edit_start_inscriptions_end(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    _ => ()
                }},

            Interaction::MessageComponent(mci) => {
                match mci.data.custom_id.as_str() {
                    DELETE_EDITION_MODAL => delete_edition_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    EDIT_START_INSCRIPTIONS => edit_start_inscriptions(MONGOCLIENT.get().unwrap(), mci, ctx, StartRegistration).await,
                    EDIT_END_INSCRIPTIONS => edit_start_inscriptions(MONGOCLIENT.get().unwrap(), mci, ctx, EndRegistration).await,
                    EDIT_START_COMPETITION => edit_start_inscriptions(MONGOCLIENT.get().unwrap(), mci, ctx, StartCompetition).await,
                    EDIT_END_COMPETITION => edit_start_inscriptions(MONGOCLIENT.get().unwrap(), mci, ctx, EndCompetition).await,
                    EDITION_SELECT => get_edition_end(MONGOCLIENT.get().unwrap(), mci, ctx).await,
                    SETUP_ENV => println!("ok setup"),
                    IMPORT_ENV => println!("ok import"),
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