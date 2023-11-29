use std::cmp::{max, min};
use std::sync::Arc;
use std::time::Duration;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::client::Context;
use serenity::model::application::command::{Command, CommandOptionType};
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use chrono;
use chrono::{NaiveDate, NaiveDateTime};
use serenity::model::application::component::{ActionRowComponent, InputTextStyle};
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::Permissions;
use crate::*;
use crate::common_functions::{get_edition_by_name, get_editions_names, send_error_from_command, send_error_from_modal, send_success_from_modal};
use crate::constants::*;

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
                    .add_string_choice("Date de début des inscriptions", EDIT_START_INSCRIPTIONS)
                    .add_string_choice("Date de fin des inscriptions", EDIT_END_INSCRIPTIONS)
                    .add_string_choice("Date de début de la compétition", EDIT_START_COMPETITION)
                    .add_string_choice("Date de fin de la compétition", EDIT_END_COMPETITION)
            })
    })
        .await;
}

pub async fn edit_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context){
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

pub async fn edit_start_inscriptions(client : &MongoClient, mci : MessageComponentInteraction, ctx : serenity::client::Context, type_modif : TypeDate){
    let edition = mci.data.values[0].clone();
    let id = mci.user.id.to_string();
    let result = get_edition_by_name(client, &edition, &id).await.unwrap();
    let date_fin_inscription = result.get(INSCRIPTION_END_DATE).unwrap().as_i64().unwrap();
    let date_start_competition = result.get(COMPETITION_START_DATE).unwrap().as_i64().unwrap();
    let date_start_inscription = result.get(INSCRIPTION_START_DATE).unwrap().as_i64().unwrap();
    let date_end_competition = result.get(COMPETITION_END_DATE).unwrap().as_i64().unwrap();
    let date_min = min(&date_start_competition, &date_fin_inscription).clone();
    let date_max = max(&date_start_competition, &date_fin_inscription).clone();

    let msg;
    let mut type_modif_str : &str = "";

    match type_modif {
        TypeDate::StartRegistration => {
            let date1 = NaiveDateTime::from_timestamp_opt(date_min, 0).unwrap().format("%d/%m/%Y").to_string();
            msg = format!("JJ/MM/AAAA (< {})", date1.clone());
            type_modif_str = INSCRIPTION_START_DATE;
        }
        TypeDate::EndCompetition => {
            let date1 = NaiveDateTime::from_timestamp_opt(date_max, 0).unwrap().format("%d/%m/%Y").to_string();
            msg = format!("JJ/MM/AAAA (> {})", date1.clone());
            type_modif_str = COMPETITION_END_DATE;
        }
        _ => {
            let date1 = NaiveDateTime::from_timestamp_opt(date_start_inscription, 0).unwrap().format("%d/%m/%Y").to_string();
            let date2 = NaiveDateTime::from_timestamp_opt(date_end_competition, 0).unwrap().format("%d/%m/%Y").to_string();
            msg = format!("({} <) JJ/MM/AAAA (< {})", &date1.clone(), &date2.clone());
            match type_modif {
                TypeDate::EndRegistration => {type_modif_str = INSCRIPTION_END_DATE;}
                TypeDate::StartCompetition => { type_modif_str = COMPETITION_START_DATE;}
                _ => {}
            }
        }
    }

    let custom_id = chrono::Utc::now().timestamp_nanos().to_string();

    mci.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_input_text(|input_text| {
                            input_text
                                .custom_id(&custom_id)
                                .placeholder(&msg)
                                .min_length(10)
                                .max_length(10)
                                .required(true)
                                .label(format!("Date de {}", type_modif_str ))
                                .style(InputTextStyle::Short)
                        })
                    })
                })
                    .title("Dates de la compétition")
                    .custom_id(&custom_id)
            )
    })
        .await
        .expect("Failed to send interaction response");


    let interaction = match mci.message.await_modal_interaction(&ctx).timeout(Duration::from_secs(60)).author_id(mci.user.id).channel_id(mci.channel_id).await {
        Some(x) => {
            mci.delete_original_interaction_response(&ctx.http).await.expect("Failed to delete interaction response");
            x
        },
        None => {
            mci.message.reply(&ctx, "Timed out").await.unwrap();
            mci.delete_original_interaction_response(&ctx).await.unwrap();
            return;
        }
    };



    let new_date = match interaction
        .data
        .components
        .get(0)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it.value.clone(),
        _ => return,
    };

    match type_modif {
        TypeDate::StartRegistration => {
            if new_date.to_string() > date_min.to_string() {
                let msg = format!("La date de début des inscriptions ne peut pas être supérieure à la date de fin des inscriptions ou de début de la compétition.\
                    \n\nRéessaye en entrant une date inférieure à **{}**", NaiveDateTime::from_timestamp_opt(date_min, 0).unwrap().format("%d/%m/%Y").to_string());
                send_error_from_modal(&interaction, &ctx, &msg).await;
                return;
            }
        }
        TypeDate::EndCompetition => {
            if new_date.to_string() < date_max.to_string() {
                let msg = format!("La date de de fin de la copétition ne peut pas être inférieure à la date de fin des inscriptions ou de début de la compétition.\
                    \n\nRéessaye en entrant une date supérieure à **{}**", NaiveDateTime::from_timestamp_opt(date_max, 0).unwrap().format("%d/%m/%Y").to_string());
                send_error_from_modal(&interaction, &ctx, &msg).await;
                return;
            }
        }
        TypeDate::EndRegistration => {
            if new_date.to_string() > date_start_inscription.to_string() && new_date.to_string() < date_end_competition.to_string() {
                let msg = format!("La date de fin des inscriptions ne peut pas être inférieure à la date de début des inscriptions.\
                    \n\nRéessaye en entrant une date supérieure à **{}**", NaiveDateTime::from_timestamp_opt(date_start_inscription, 0).unwrap().format("%d/%m/%Y").to_string());
                send_error_from_modal(&interaction, &ctx, &msg).await;
                return;
            }
        }
        TypeDate::StartCompetition => {
            if new_date.to_string() > date_start_inscription.to_string()  && new_date.to_string() < date_end_competition.to_string(){
                let msg = format!("La date de début des inscriptions ne peut pas être supérieure à la date de fin des inscriptions.\
                    \n\nRéessaye en entrant une date inférieure à **{}**", NaiveDateTime::from_timestamp_opt(date_end_competition, 0).unwrap().format("%d/%m/%Y").to_string());
                send_error_from_modal(&interaction, &ctx, &msg).await;
                return;
            }
        }
    }

    let date = NaiveDate::parse_from_str(&new_date, "%d/%m/%Y").unwrap().and_hms_micro_opt(0, 0, 0, 0).unwrap().timestamp();
    update_date(&ctx, &interaction, &client, &type_modif_str, &date, &edition).await;
}

async fn update_date(ctx: &Context, mci: &Arc<ModalSubmitInteraction>, client : &MongoClient, type_modif: &str, date: &i64, edition : &str) {
    client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).update_one(
        doc! {
                EDITION_NAME: &edition,
                ORGANIZER: mci.user.id.to_string()
            },
        doc! {
                "$set": doc! {
                    type_modif: date
                }
            },
        None
    ).await.unwrap();

    let msg = format!("La date de début des inscriptions de la compétition **{}** a bien été modifiée.", &edition);
    send_success_from_modal(&mci, &ctx, "Date modifiée avec succès", &msg).await;
}