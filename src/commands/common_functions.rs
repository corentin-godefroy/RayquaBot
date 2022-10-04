use std::sync::Arc;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use crate::doc;

pub const EDITIONS_COLLECTION: &str = "editions";
pub const RAYQUABOT_DB : &str = "RayquaBot";
pub const DATE_DEBUT_INSCRIPTION : &str = "date_debut_inscription";
pub const DATE_FIN_INSCRIPTION : &str = "date_fin_inscription";
pub const DATE_DEBUT_COMPETITION : &str = "date_debut_competition";
pub const DATE_FIN_COMPETITION : &str = "date_fin_competition";
pub const GUILD_ID : &str = "guild_id";
pub const NOM_EDITION : &str = "nom_edition";
pub const ORGANISATEUR : &str = "organisateur";
pub const RED_COLOR : i32 = 0xff0000;
pub const GREEN_COLOR : i32 = 0x00ff00;
pub const CONTACT : &str = "contact.cgbots@gmail.com";

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

pub async fn send_error_from_message (mci : &ApplicationCommandInteraction, ctx : &Context, err : &str) {
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
