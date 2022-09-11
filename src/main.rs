//Bot discord en rust

pub mod commands;
pub mod mongo_functions;

use std::borrow::Borrow;
use commands::*;
use mongo_functions::*;


use std::collections::HashMap;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use mongodb::bson::Bson::Null;
use mongodb::bson::doc;
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::framework::StandardFramework;
use serenity::json::Value;
use serenity::model::application::command::{Command, CommandPermission};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::{Channel, ChannelCategory, GuildChannel, PartialGuildChannel, Reaction, StageInstance};
use serenity::model::event::{ChannelPinsUpdateEvent, GuildMembersChunkEvent, GuildMemberUpdateEvent, GuildScheduledEventUserAddEvent, GuildScheduledEventUserRemoveEvent, InviteCreateEvent, InviteDeleteEvent, MessageUpdateEvent, ResumedEvent, ThreadListSyncEvent, ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent};
use serenity::model::gateway::Presence;
use serenity::model::guild::automod::{ActionExecution, Rule};
use serenity::model::guild::{Emoji, Guild, Integration, Member, PartialGuild, Role, ScheduledEvent, ThreadMember, UnavailableGuild};
use serenity::model::id::{ApplicationId, ChannelId, EmojiId, GuildId, IntegrationId, MessageId, RoleId, StickerId};
use serenity::model::prelude::{CurrentUser, Sticker, User, VoiceState};

use mongodb::{Client as MongoClient, Database};
use mongodb::options::{ClientOptions as MongoClientOptions, ClientOptions};
use crate::commands::commands::{new_edition_reactor, new_edition_setup, ping_reactor, ping_setup};
use crate::mongo_functions::mongo_functions::new_edition_insertion;

use once_cell::sync::OnceCell;

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

    let uri = std::env::var("MONGODB_LOGIN").unwrap();
    let client = MongoClient::with_uri_str(&uri).await.unwrap();
    CLIENT.set(client).unwrap();

    let token = std::env::var("DISCORD_TOKEN").expect("token");

    let mut client = Client::builder(&token, Default::default())
        .event_handler(HandlerDiscord)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
