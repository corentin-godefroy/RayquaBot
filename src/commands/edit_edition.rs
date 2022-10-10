use std::time::Duration;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::{CreateEmbed};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::client::Context;
use serenity::model::application::command::{Command, CommandOptionType};
use mongodb::{Client as MongoClient, Client};
use mongodb::bson::Document;
use chrono;
use chrono::{NaiveDate, NaiveDateTime};
use mongodb::bson::Bson::DateTime;
use serenity::futures::StreamExt;
use serenity::model::application::component::{ActionRowComponent, InputTextStyle};
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::Permissions;
use crate::doc;
use super::common_functions::*;

pub async fn edit_edition_setup(ctx : &Context){
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("edit_edition")
            .description("Permet d'éditer une édition actuelle ou future.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
            .create_option(|option| {
                option
                    .name("edition")
                    .description("Nom de l'édition à éditer.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("Date de début des inscriptions", "edit_start_inscriptions")
                    .add_string_choice("Date de fin des inscriptions", "edit_end_inscriptions")
                    .add_string_choice("Date de début de la compétition", "edit_start_competition")
                    .add_string_choice("Date de fin de la compétition", "edit_end_competition")
            })
    })
        .await;
}
/*
pub async fn edit_edition(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context){

}*/

pub async fn edit_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context){
    let ctx = context;
    let com = &command.clone();

    let user_id = com.user.id.to_string();

    let editions = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                "organisateur": user_id,
                "date_fin_competition": doc! {
                    "$gt": chrono::Utc::now().timestamp()
                }
            }
        },
            doc! {
                "$project": doc! {
                    "nom_edition": 1
                }
            }
        ]
        , None).await.unwrap().collect::<Vec<_>>().await;

    let mut opt = Vec::new();

    for edition in editions {
        let edition = edition.unwrap().get(NOM_EDITION).unwrap().as_str().unwrap().to_string();
        opt.push(edition);
    }

    if opt.is_empty() {
        let msg = format!("Vous n'avez aucune édition en cours ou future qui puisse être modifiée.\
        \nVous ne pouvez pas modifier une édition qui a déjà eu lieu.\
        \n\nPour toute demande de modification d'édition passée, veuillez contacter le développeur à l'adresse mail **{}**", CONTACT);
        send_error_from_message(&com, &ctx, &msg).await;
    }

    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(command.data.options[0].value.as_ref().unwrap().as_str().unwrap())
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

pub async fn edit_start_inscriptions(client : &MongoClient, mci : MessageComponentInteraction, ctx : serenity::client::Context){
    let edition = mci.data.values[0].clone();
    let result = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).find_one(
        doc! {
            "nom_edition": &edition,
            "organisateur": mci.user.id.to_string()
        },
        None
    ).await.unwrap().unwrap();

    let date_fin_inscription = result.get("date_fin_inscription").unwrap().as_i64().unwrap();
    let date = NaiveDateTime::from_timestamp(date_fin_inscription, 0).format("%d/%m/%Y").to_string();

    mci.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_input_text(|input_text| {
                            input_text
                                .custom_id("edit_start_inscriptions_end")
                                .placeholder(format!("JJ/MM/AAAA (< {})", date))
                                .min_length(10)
                                .max_length(10)
                                .required(true)
                                .label("Date de début et de fin de la compétition.")
                                .style(InputTextStyle::Short)
                        })
                    })
                })
                    .title("Dates de la compétition")
                    .custom_id("edit_start_inscriptions_end")
            )
    })
        .await
        .expect("Failed to send interaction response");

    let interaction =
        match mci.message.await_modal_interaction(&ctx).timeout(Duration::from_secs(60)).await {
            Some(x) => {
                mci.message.delete(&ctx).await.unwrap();
                x
            },
            None => {
                mci.message.reply(&ctx, "Timed out").await.unwrap();
                mci.delete_original_interaction_response(&ctx).await.unwrap();
                return;
            }
        };

    let date = match interaction
        .data
        .components
        .get(0)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let date = NaiveDate::parse_from_str(&date.value, "%d/%m/%Y").unwrap();
    let date = NaiveDateTime::new(date, chrono::NaiveTime::from_hms(0, 0, 0));
    let date = date.timestamp();

    let date_fin_inscription = result.get("date_fin_inscription").unwrap().as_i64().unwrap();

    if date_fin_inscription < date {
        let msg = format!("La date de début des inscriptions ne peut pas être supérieure à la date de fin des inscriptions.\
        \n\nVeuillez réessayer en entrant une date inférieure à **{}**", NaiveDateTime::from_timestamp(date_fin_inscription, 0).format("%d/%m/%Y").to_string());
        send_error_from_modal(&interaction, &ctx, &msg).await;
    } else {
        client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).update_one(
            doc! {
                "nom_edition": &edition,
                "organisateur": mci.user.id.to_string()
            },
            doc! {
                "$set": doc! {
                    "date_debut_inscription": date
                }
            },
            None
        ).await.unwrap();

        let msg = format!("La date de début des inscriptions de la compétition **{}** a bien été modifiée.", &edition);
        send_success_from_modal(&interaction, &ctx, "Date modifiée avec succès", &msg).await;
    }
}