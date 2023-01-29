use std::ops::{Add, Sub};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::TimeZone;
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use crate::commands::constants::{BDD_POKE_LEGENDARCEUS, BLUE_COLOR, GREEN_COLOR, PING};


pub async fn ping_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name(PING).description("Reply with Pong")
    })
        .await.expect("Creation of ping command failed : ");
}

pub async fn ping_reactor(command: &ApplicationCommandInteraction, ctx: &Context) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
    let delay = (time as i64) .sub(((command.id.0 >> 22) + 1420070400000) as i64);
    command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data|{
                    data.embed(|embed|{
                        embed.colour(BLUE_COLOR)
                            .title("Pong !")
                            .description(format!("{} ms", delay))
                    })
                })
        })
        .await
        .expect("Failed to send interaction response");
}