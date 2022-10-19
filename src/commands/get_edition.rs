use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use chrono;
use chrono::{NaiveDateTime};
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::Permissions;
use crate::*;
use super::common_functions::*;

pub async fn get_edition_setup(ctx : &Context){
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(GET_EDITION)
            .description("Permet d'afficher les informations d'une édition.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await;
}

pub async fn get_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context){
    let ctx = context;
    let com = &command.clone();
    let user_id = com.user.id.to_string();
    let editions = get_editions_names(client, &user_id).await;
    let mut opt = Vec::new();

    for edition in editions {
        let edition = edition.unwrap().get(EDITION_NAME).unwrap().as_str().unwrap().to_string();
        opt.push(edition);
    }

    if opt.is_empty() {
        let msg = format!("Vous n'avez aucune édition en cours ou future qui puisse être modifiée.\
        \nVous ne pouvez pas modifier une édition qui a déjà eu lieu.\
        \n\nPour toute demande de modification d'édition passée, veuillez contacter le développeur à l'adresse mail **{}**", CONTACT);
        send_error_from_command(&com, &ctx, &msg).await;
    }

    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(EDITION_SELECT)
                                .placeholder("Choisissez une édition")
                                .options(|options| {
                                    for option in opt {
                                        options.create_option(|select_menu_option| {
                                            select_menu_option.label(&option)
                                                .value(&option)
                                        });
                                    }
                                    options
                                })
                        })
                    })
                })
            )
    })
        .await
        .expect("Failed to send interaction response");
}

pub async fn get_edition_end(client : &MongoClient, mci : MessageComponentInteraction, ctx : serenity::client::Context){
    let edition = mci.data.values[0].clone();
    let result = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).find_one(
        doc! {
            EDITION_NAME: &edition,
            ORGANISATOR: mci.user.id.to_string()
        },
        None
    ).await.unwrap().unwrap();

    let edition_name = result.get(EDITION_NAME).unwrap().as_str().unwrap();
    let inscription_start_date = result.get(INSCRIPTION_START_DATE).unwrap().as_i64().unwrap();
    let inscription_end_date = result.get(INSCRIPTION_END_DATE).unwrap().as_i64().unwrap();
    let competition_start_date = result.get(COMPETITION_START_DATE).unwrap().as_i64().unwrap();
    let competition_end_date = result.get(COMPETITION_END_DATE).unwrap().as_i64().unwrap();

    let _ = mci.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|message|
                message.embed(|embed| {
                    embed.title(&edition)
                        .description(format!("Voici les informations de l'édition {}", edition_name))
                        .field("Date de début des inscriptions" , &format!("{}", NaiveDateTime::from_timestamp(inscription_start_date, 0).format("%d/%m/%Y")), false)
                        .field("Date de fin des inscriptions"   , &format!("{}", NaiveDateTime::from_timestamp(inscription_end_date  , 0).format("%d/%m/%Y")), false)
                        .field("Date de début de la compétition", &format!("{}", NaiveDateTime::from_timestamp(competition_start_date, 0).format("%d/%m/%Y")), false)
                        .field("Date de fin de la compétition"  , &format!("{}", NaiveDateTime::from_timestamp(competition_end_date  , 0).format("%d/%m/%Y")), false)
                })
            )
    })
        .await
        .expect("Failed to send interaction response");
}