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
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::prelude::User;
use tokio::join;

use crate::commands::*;
use constants::TypeDate::{EndCompetition, EndRegistration, StartCompetition, StartRegistration};
use crate::commands::edition::print_edition_parameters::{print_versions, print_versions_reactor, print_versions_setup};
use crate::commands::edition::version_setup::{version_setup_end, version_setup_reactor, version_setup};
use crate::commands::joueurs::registration::{get_registration_reactor, registration_setup};
use crate::commands::joueurs::versions_player_setup::{version_player_setup, version_player_setup_end, version_player_setup_reactor};
use crate::commands::setup_env_bot::{setup_env, setup_env_setup};
use crate::delete_edition::*;
use crate::edit_edition::*;
use crate::edition::*;
use crate::get_edition::*;
use crate::new_edition::*;
use crate::constants::*;

//global variable for mongodb client
static MONGOCLIENT: OnceCell<MongoClient> = OnceCell::new();

//handler discord, bot core
struct HandlerDiscord;
#[async_trait]
impl EventHandler for HandlerDiscord {
    async fn ready(&self, ctx: Context, ready: Ready) {
        /*let commands = ctx.http.get_global_application_commands().await.unwrap();
        for command in commands{
            if command.name == VERSION_SETUP {
                ctx.http.delete_global_application_command(command.id.0).await.unwrap();
            }
        }*/
        let ping = ping_setup(ctx.borrow());
        let new_edition = new_edition_setup(ctx.borrow());
        let delete_edition = delete_edition_setup(ctx.borrow());
        let edit_edition = edit_edition_setup(ctx.borrow());
        let get_edition = get_edition_setup(ctx.borrow());
        let setup_env = setup_env_setup(ctx.borrow());
        let registration = registration_setup(ctx.borrow());
        let version_setup = version_setup(ctx.borrow());
        let print_versions = print_versions_setup(ctx.borrow());
        let version_player_setup = version_player_setup(ctx.borrow());
        join!(ping, new_edition, delete_edition, edit_edition, get_edition, setup_env, registration, version_setup, version_player_setup);
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction{
            Interaction::ApplicationCommand(command) => {
                match command.data.name.as_str() {
                    PING => ping_reactor(&command, &ctx).await,
                    SETUP_ENV => setup_env(&ctx, &command, MONGOCLIENT.get().unwrap()).await,
                    NEW_EDITION => new_edition(&command, &ctx, MONGOCLIENT.get().unwrap()).await,
                    DELETE_EDITION => delete_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    EDIT_EDITION => edit_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    GET_EDITION => get_edition_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    REGISTRATION => get_registration_reactor(MONGOCLIENT.get().unwrap(), &command, &ctx).await,
                    VERSION_SETUP => version_setup_reactor(&command, &ctx, MONGOCLIENT.get().unwrap()).await,
                    PRINT_VERSIONS => print_versions_reactor(&command, &ctx, MONGOCLIENT.get().unwrap()).await,
                    VERSION_PLAYER_SETUP => version_player_setup_reactor(&command, &ctx, MONGOCLIENT.get().unwrap()).await,
                    _ => ()
                }},

            Interaction::ModalSubmit(mci) => {
                match mci.data.custom_id.as_str() {
                    CREATE_NEW_EDITION => new_edition_modal(MONGOCLIENT.get().unwrap(), mci, ctx).await,
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
                    VALIDATE => println!("OK"),
                    _ => if mci.data.custom_id.as_str().starts_with(LOCK_VERSION_MODAL) {
                        version_setup_end(&mci, &ctx, MONGOCLIENT.get().unwrap()).await
                    }
                    else if mci.data.custom_id.as_str().starts_with(VERSION_PLAYER_MODAL) {
                        version_player_setup_end(&mci, &ctx, MONGOCLIENT.get().unwrap()).await
                    }
                    PRINT_VERSIONS_MODAL => print_versions(&mci, &ctx, MONGOCLIENT.get().unwrap()).await
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