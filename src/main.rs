//Bot discord en rust

use std::collections::HashMap;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
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

use mongodb::Client as MongoClient;
use mongodb::options::ClientOptions as MongoClientOptions;


struct HandlerDiscord;
#[async_trait]
impl EventHandler for HandlerDiscord {
    //create global application ping command
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Reply with Pong")
        })
            .await.expect("Creation of ping command failed : ");
    }

    //recieve interaction for ping
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name == "ping" {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("Pong!"))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {:?}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Parse a connection string into an options struct.
    let link = std::env::var("MONGODB_LOGIN").expect("login to mongodb database not found");
    let mut mongo_client_options = MongoClientOptions::parse(link).await.expect("Failed to parse client options.");

    // Manually set an option.
    mongo_client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let mongo_client = MongoClient::with_options(mongo_client_options).unwrap();

    // List the names of the databases in that deployment.
    for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
        println!("{}", db_name);
    }



    //login with a bot token from the environment
    //dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("token");

    //create a new client
    let mut client = Client::builder(&token, Default::default())
        .event_handler(HandlerDiscord)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
