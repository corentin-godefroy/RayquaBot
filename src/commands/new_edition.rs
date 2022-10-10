use std::borrow::Borrow;
use std::sync::Arc;
use std::time::Duration;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::{CreateActionRow, CreateComponents, CreateEmbed, CreateInputText};
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient};
use mongodb::bson::{bson, Document};
use chrono;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serenity::futures::future::err;
//use mongodb::error::ErrorKind::Command as cmd;

use serenity::model::application::component::InputTextStyle;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::model::Permissions;
use crate::commands::common_functions::{send_error_from_component, send_error_from_modal};

use crate::doc;

const EDITIONS_COLLECTION: &str = "editions";
const RAYQUABOT_DB : &str = "RayquaBot";
const DATE_DEBUT_INSCRIPTION : &str = "date_debut_inscription";
const DATE_FIN_INSCRIPTION : &str = "date_fin_inscription";
const DATE_DEBUT_COMPETITION : &str = "date_debut_competition";
const DATE_FIN_COMPETITION : &str = "date_fin_competition";
const GUILD_ID : &str = "guild_id";
const NOM_EDITION : &str = "nom_edition";
const ORGANISATEUR : &str = "organisateur";
const RED_COLOR : i32 = 0xff0000;
const GREEN_COLOR : i32 = 0x00ff00;

struct DATE {
    jour : u8,
    mois : u8,
    annee : u16
}

impl ToString for DATE {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:04}", self.jour, self.mois, self.annee)
    }
}

impl Clone for DATE {
    fn clone(&self) -> Self {
        DATE {
            jour: self.jour,
            mois: self.mois,
            annee: self.annee
        }
    }
}

pub async fn new_edition_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("new_edition")
            .description("Créé une nouvelle édition.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await.unwrap();
}

pub async fn new_edition(command : &ApplicationCommandInteraction, ctx : &Context) {

    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
                    components
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id("nom")
                                    .placeholder("Le couple nom/numéro doit être différent des anciennes éditions")
                                    .min_length(4)
                                    .max_length(50)
                                    .required(true)
                                    .label("Nom de l'édition avec un numéro (ou année).")
                                    .style(InputTextStyle::Short)
                            })
                        })
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id("inscription")
                                    .placeholder("JJ/MM/AAAA-JJ/MM/AAAA")
                                    .min_length(21)
                                    .max_length(21)
                                    .required(true)
                                    .label("Date de début et de fin des inscription")
                                    .style(InputTextStyle::Short)
                            })
                        })
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id("competition")
                                    .placeholder("JJ/MM/AAAA-JJ/MM/AAAA")
                                    .min_length(21)
                                    .max_length(21)
                                    .required(true)
                                    .label("Date de début et de fin de la compétition")
                                    .style(InputTextStyle::Short)
                            })
                        })
                })
                    .title("Dates de la compétition")
                    .custom_id("create_new_edition")
            )
    })
        .await
        .expect("Failed to send interaction response");


}

pub async fn new_edition_modal(client : &MongoClient, mci : ModalSubmitInteraction, ctx : serenity::client::Context) {
    let nom_competition = match mci
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

    let date_inscription = match mci
        .data
        .components
        .get(1)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let date_competition = match mci
        .data
        .components
        .get(2)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let mut date_inscription = parsing_two_dates(date_inscription.value.as_str());
    let date_competition = parsing_two_dates(date_competition.value.as_str());
    let guild = mci.guild_id.unwrap();
    let organisateur = mci.user.id.0.to_string();

    match date_inscription {
        Ok(ref dates) => { let date_inscription = &dates;}
        Err(ref e) => { send_error_from_modal(&mci, &ctx, &e).await;
            return;
        }
    }
    match date_competition {
        Ok(ref dates) => { let date_competition = &dates;}
        Err(ref e) => { send_error_from_modal(&mci, &ctx, &e).await;
            return;
        }
    }

    let timestamp_debut_inscription = get_timestamp_from_date(&date_inscription.as_ref().unwrap().0,  &mci, &ctx).await;
    let timestamp_fin_inscription = get_timestamp_from_date(&date_inscription.as_ref().unwrap().1, &mci, &ctx).await;
    let timestamp_debut_competition = get_timestamp_from_date(&date_competition.as_ref().unwrap().0, &mci, &ctx).await;
    let timestamp_fin_competition = get_timestamp_from_date(&date_competition.as_ref().unwrap().1, &mci, &ctx).await;

    let resultat = verification_chevauchement_edition(&timestamp_debut_inscription, &timestamp_fin_competition, &client, &guild, &ctx, &mci).await;

    match resultat {
        Ok(_) => {
            let collection =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
            let doc = doc! {
                ORGANISATEUR: organisateur,
                NOM_EDITION : nom_competition.value.as_str(),
                GUILD_ID    : &guild.0.to_string(),
                DATE_DEBUT_INSCRIPTION : &timestamp_debut_inscription,
                DATE_FIN_INSCRIPTION   : &timestamp_fin_inscription,
                DATE_DEBUT_COMPETITION : &timestamp_debut_competition,
                DATE_FIN_COMPETITION   : &timestamp_fin_competition
            };
            collection.insert_one(doc, None).await.expect("Failed to insert document");

            mci.create_interaction_response(ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        d.add_embed(
                            CreateEmbed::default()
                                .title(format!("Nouvelle compétition {} créée avec succès !", nom_competition.value.as_str()))
                                .description(format!(
                                    "Les inscriptions démarerons le {}\net se fermerons le {}.\n\
                                    La compétition commenceras de {}\net se termineras le {}.",
                                    NaiveDateTime::from_timestamp(timestamp_debut_inscription.to_owned(), 0).format("%B %e %Y"),
                                    NaiveDateTime::from_timestamp(timestamp_fin_inscription.to_owned(), 0).format("%B %e %Y"),
                                    NaiveDateTime::from_timestamp(timestamp_debut_competition.to_owned(), 0).format("%B %e %Y"),
                                    NaiveDateTime::from_timestamp(timestamp_fin_competition.to_owned(), 0).format("%B %e %Y"),
                                )).to_owned()
                                .color(GREEN_COLOR)
                                .to_owned()
                        )
                    })
            })
                .await
                .unwrap();
        }
        Err(e) => send_error_from_modal(&mci, &ctx, &e).await
    }
}

fn parsing_two_dates(date : &str) -> Result<(DATE, DATE), String> {
    let dates : Vec<String> = date.split("-").map(|s| s.to_string()).collect();
    if dates.len() != 2 { return Err(format!("les dates {} sont mal écrites. Respecte bien le format JJ/MM/AAAA-JJ/MM/AAAA", date).to_string())}

    let date1 = dates.get(0).unwrap();
    let date1 = parse_one_date(date1).unwrap();


    let date2= dates.get(1).unwrap();
    let date2 = parse_one_date(date2).unwrap();
    Ok((date1, date2))
}

fn parse_one_date(date : &str) -> Result<DATE, String> {
    let date : Vec<String> = date.split("/").map(|s| s.to_string()).collect();
    if date.len() != 3 {return Err("La date n'est pas correctement écrite. Respecte l'écriture suivante : JJ/MM/AAAA-JJ/MM/AAAA".to_string());}
    let date = DATE {
        jour : date[0].parse::<u8>().unwrap(),
        mois : date[1].parse::<u8>().unwrap(),
        annee : date[2].parse::<u16>().unwrap()
    };
    Ok(date)
}

async fn verification_chevauchement_edition(timestamp_debut : &i64, timestamp_fin : &i64, client : &MongoClient, guild_id : &GuildId, ctx : &Context, mci : &ModalSubmitInteraction) ->Result<(), String>{
    if timestamp_debut.to_owned() == 0 || timestamp_fin.to_owned() == 0 {
        return if timestamp_fin.to_owned() == 0 {
            Err("la date de fin n'est pas valide !".to_string())
        } else {
            Err("la date de fin n'est pas valide !".to_string())
        }
    }

    if timestamp_fin.to_owned() < timestamp_debut.to_owned() {
        return Err("La date de fin est avant la date de début !".to_string())
    }
    let querry = doc! {
        GUILD_ID: &guild_id.0.to_string(),
        "$nor": [
            doc! { DATE_DEBUT_INSCRIPTION: doc! {"$gt": timestamp_fin}},
            doc! { DATE_FIN_COMPETITION  : doc! {"$lt": timestamp_debut}}
        ]
    };

    let count = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).count_documents(querry,None).await.unwrap();

    if count > 0 {
        return Err("L'édition chevauche une édition existante !".to_string());
    }
    return Ok(());
}

async fn get_timestamp_from_date(date : &DATE, msi: &ModalSubmitInteraction, ctx: &Context) -> i64 {
    match NaiveDate::from_ymd_opt(date.annee as i32, date.mois as u32, date.jour as u32) {
        Some(date) => {
            let date = date.and_hms(0, 0, 0);
            let timestamp = date.timestamp();
            timestamp
        },
        None => {
            send_error_from_modal(&msi, &ctx, &format!("La date \"{}\" entrée est invalide", date.to_string())).await;
            return 0
        }
    }
}