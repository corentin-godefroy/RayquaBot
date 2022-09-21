use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use mongodb::Client;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::{CreateEmbed};
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command, CommandOptionType};
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use chrono;
use chrono::{NaiveDate, NaiveDateTime, ParseResult};
use serenity::futures::StreamExt;
use serenity::model::application::component::InputTextStyle;
use serenity::model::id::GuildId;

use crate::doc;

struct Edition {
    organisateur : String,
    nom : String,
    guild_id: String,
    date_debut_inscription: i64,
    date_fin_inscription: i64,
    date_debut_competition: i64,
    date_fin_competition: i64
}

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
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("nouvelle_edition")
            .description("Créé une nouvelle édition.")
    })
        .await;
}

pub async unsafe fn new_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context) {
    let ctx = context;
    let com = &command.clone();
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_input_text(|input_text| {
                            input_text
                                .custom_id("nom")
                                .placeholder("Le couple nom/numéro doit être différent des anciennes éditions")
                                .min_length(4)
                                .max_length(30)
                                .required(true)
                                .label("Nom de l'édition avec un numéro (ou année).")
                                .style(InputTextStyle::Short)
                        })
                    });
                    components.create_action_row(|action_row| {
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
                    });
                    components.create_action_row(|action_row| {
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
                    .custom_id("competition")
            )
    })
        .await
        .expect("Failed to send interaction response");
}

pub async fn prompt_date_debut_inscription_modal(client : &MongoClient, mci : ModalSubmitInteraction, ctx : serenity::client::Context) {
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

    let date_inscription = parsing_two_dates(date_inscription.value.as_str());
    let date_competition = parsing_two_dates(date_competition.value.as_str());
    let guild = mci.guild_id.unwrap();
    let organisateur = mci.user.id.0.to_string();


    let timestamp_debut_inscription = get_timestamp_from_DATE(&date_inscription.0);
    resolver_timestamp(&timestamp_debut_inscription, &mci, &ctx, &client).await;
    let timestamp_fin_inscription = get_timestamp_from_DATE(&date_inscription.1);
    resolver_timestamp(&timestamp_fin_inscription, &mci, &ctx, &client).await;
    let timestamp_debut_competition = get_timestamp_from_DATE(&date_competition.0);
    resolver_timestamp(&timestamp_debut_competition, &mci, &ctx, &client).await;
    let timestamp_fin_competition = get_timestamp_from_DATE(&date_competition.1);
    resolver_timestamp(&timestamp_fin_competition, &mci, &ctx, &client).await;

    verification_chevauchement_edition(&timestamp_debut_inscription.as_ref().unwrap(), &client, &guild, &ctx, &mci).await;
    verification_chevauchement_edition(&timestamp_debut_competition.as_ref().unwrap(), &client, &guild, &ctx, &mci).await;
    verification_chevauchement_edition(&timestamp_fin_inscription.as_ref().unwrap(), &client, &guild, &ctx, &mci).await;
    verification_chevauchement_edition(&timestamp_fin_competition.as_ref().unwrap(), &client, &guild, &ctx, &mci).await;

    let collection =  client.database("RayquaBot").collection("editions");
    let doc = doc! {
            "organisateur" : organisateur,
            "nom" : nom_competition.value.as_str(),
            "guild_id": &guild.0.to_string(),
            "date_debut_inscription": &timestamp_debut_inscription.unwrap(),
            "date_fin_inscription": &timestamp_fin_inscription.unwrap(),
            "date_debut_competition": &timestamp_debut_competition.unwrap(),
            "date_fin_competition": &timestamp_fin_competition.unwrap()
        };
    collection.insert_one(doc, None).await.expect("Failed to insert document");

    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Date entrée : ")
                        .description(format!(
                            "dates inscriptions : \n\
                            début : {}\n\
                            fin : {}\n\
                            dates compétition :\n\
                            début : {}\n\
                            fin : {}", date_inscription.0.to_string(), date_inscription.1.to_string(), date_competition.0.to_string(), date_competition.1.to_string())).to_owned()
                )
            })
    })
        .await
        .unwrap();
}

fn parsing_two_dates(date : &str) -> (DATE, DATE) {
    let dates : Vec<String> = date.split("-").map(|s| s.to_string()).collect();

    let date_debut = dates.get(0).unwrap();
    let date_debut = parse_one_date(date_debut);

    let date_fin= dates.get(1).unwrap();
    let date_fin = parse_one_date(date_fin);
    (date_debut, date_fin)
}

fn parse_one_date(date : &str) -> DATE {
    let date : Vec<String> = date.split("/").map(|s| s.to_string()).collect();
    let date = DATE {
        jour : date[0].parse::<u8>().unwrap(),
        mois : date[1].parse::<u8>().unwrap(),
        annee : date[2].parse::<u16>().unwrap()
    };
    date
}


async fn verification_chevauchement_edition(timestamp : &i64, client : &MongoClient, guild_id : &GuildId, ctx : &Context, mci : &ModalSubmitInteraction) {
    //todo FIXIT
    let count = client.database("RayquaBot").collection::<Document>("edition").aggregate(
        [
            doc! {
                "$match": doc! {
                    "$and": [
                        doc! {
                            "date_debut_inscription": doc! {
                                "$lt": timestamp
                            }
                        },
                        doc! {
                            "date_fin_competition": doc! {
                                "$gt": timestamp
                            }
                        },
                        doc! {
                            "guild_id": doc! {
                                "$eq": guild_id.0.to_string()
                            }
                        }
                    ]
                }
            },
        ], None).await.unwrap().count().await;
    if count > 0 {
        send_error(&mci, &ctx, "L'édition chevauche une édition existante !").await;
        panic!("Chevauchement d'édition !")
    }
}

async fn send_error (mci : &ModalSubmitInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Erreur !")
                        .description(err)
                        .color(0xff0000)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}

fn get_timestamp_from_DATE(date : &DATE) -> Result<i64, String> {
    match NaiveDate::from_ymd_opt(date.annee as i32, date.mois as u32, date.jour as u32){
        Some(date) => {
            let date = date.and_hms(0, 0, 0);
            let timestamp = date.timestamp();
            Ok(timestamp)
        },
        None => Err("Date invalide".to_string())
    }
}

async fn resolver_timestamp(t : &Result<i64, String>, mci : &ModalSubmitInteraction, ctx : &Context, db : &MongoClient) {
    match t {
        Err(err) => {
            send_error( &mci, &ctx, &err).await;
            panic!("Date invalide !")
        },
        _ => ()
    }
}