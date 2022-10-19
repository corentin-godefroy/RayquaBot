use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use crate::doc;
use crate::commands::constants::*;
use serenity::futures::StreamExt;

pub async fn send_error_from_modal (mci : &ModalSubmitInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}


pub async fn send_error_from_command (mci : &ApplicationCommandInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

pub async fn send_error_from_component (mci : &MessageComponentInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

pub async fn reset_global_application_command(ctx : &Context){
    let commands = ctx.http.get_global_application_commands().await.unwrap();
    for command in commands{
        ctx.http.delete_global_application_command(command.id.0).await.unwrap();
    }
}

pub async fn send_success_from_component(mci : &MessageComponentInteraction, ctx : &Context, title : &str, message : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title(title)
                        .description(message)
                        .color(GREEN_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

pub async fn send_success_from_modal(mci : &ModalSubmitInteraction, ctx : &Context, title : &str, message : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title(title)
                        .description(message)
                        .color(GREEN_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

pub async fn send_success_from_command (mci : &ApplicationCommandInteraction, ctx : &Context, title : &str, message : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title(title)
                        .description(message)
                        .color(GREEN_COLOR)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

pub async fn get_edition_by_name(client : &MongoClient, name : &str, id : &str) -> Option<Document> {
    client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).find_one(
        doc! {
            EDITION_NAME: &name,
            ORGANISATOR: &id
        },
        None
    ).await.unwrap()
}

pub async fn get_editions_names(client : &MongoClient, id : &str) -> Vec<Result<mongodb::bson::Document, mongodb::error::Error>> {
    client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                ORGANISATOR: id,
                COMPETITION_END_DATE: doc! {
                    "$gt": chrono::Utc::now().timestamp()
                }
            }
        },
            doc! {
                "$project": doc! {
                    EDITION_NAME: 1
                }
            }
        ]
        , None).await.unwrap().collect::<Vec<_>>().await
}