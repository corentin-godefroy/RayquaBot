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

use serenity::model::application::component::InputTextStyle;

use crate::doc;


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
            .name("new_edition")
            .description("Create a new edition")
            .create_option(|option| {
                option
                    .name("name")
                    .description("Name of the edition")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("number")
                    .description("Number of the edition")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    })
        .await;
}

pub async fn new_edition_insertion(client : &Client, command : &ApplicationCommandInteraction) {
    let name = command.data.options.get(0).unwrap().value.as_ref().unwrap().to_string();
    let numero = command.data.options.get(1).unwrap().value.as_ref().unwrap().to_string();
    let organisateur = command.user.id.0.to_string();
    let guild = command.guild_id.unwrap().0.to_string();

    let collection =  client.database("RayquaBot").collection("editions");
    let doc = doc! {
            "organisateur" : organisateur,
            "nom" : name,
            "numero" : numero,
            "guild_id": &guild,
            "time_zone" : "",
            "date_debut_inscription": "",
            "date_fin_inscription": "",
            "date_debut_competition": "",
            "date_fin_competition": ""
        };
    collection.insert_one(doc, None).await.expect("Failed to insert document");
}

pub async unsafe fn new_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context) {
    let ctx = context;
    let com = &command.clone();
    new_edition_insertion(client, com).await;
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
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
    let date_inscription = match mci
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

    let date_competition = match mci
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

    let date_inscription = parsing_two_dates(date_inscription.value.as_str());
    let date_competition = parsing_two_dates(date_competition.value.as_str());
    let guild = mci.guild_id.unwrap().to_string();

    let collection =  client.database("RayquaBot").collection::<Document>("editions");

    //todo Vérifier qu'il n'existe pas déjà une édition sur le même serveur a la même periode
    collection.find(doc! {"guild_id": &guild}, None).await.expect("Failed to find document");

    match verification_existance_date(&date_inscription.0){
        Err(e) => {
            send_error(&mci, &ctx, &e).await;
            collection.delete_one(doc! {"guild_id": guild}, None).await.expect("Failed to delete document");
            return;
        },
        _ => ()
    }

    match verification_existance_date(&date_inscription.1){
        Err(e) => {
            send_error(&mci, &ctx, &e).await;
            collection.delete_one(doc! {"guild_id": guild}, None).await.expect("Failed to delete document");
            return;
        },
        _ => ()
    }

    match verification_existance_date(&date_competition.0){
        Err(e) => {
            send_error(&mci, &ctx, &e).await;
            collection.delete_one(doc! {"guild_id": guild}, None).await.expect("Failed to delete document");
            return;
        },
        _ => ()
    }

    match verification_existance_date(&date_competition.1){
        Err(e) => {
            send_error(&mci, &ctx, &e).await;
            collection.delete_one(doc! {"guild_id": guild}, None).await.expect("Failed to delete document");
            return;
        },
        _ => ()
    }

    let validite_dates = verifications_dates(&date_inscription, &date_competition);
    match validite_dates{
        "ok" => (),
        _ => { send_error(&mci, &ctx, validite_dates).await;
            collection.delete_one(doc! {"guild_id": guild}, None).await.expect("Failed to delete document");
            return;
        }
    }

    collection.update_one(
            doc! {"guild_id": guild},
            doc! {
                "$set": {
                    "date_debut_inscription": format!("{}", date_inscription.0.to_string()),
                    "date_fin_inscription": format!("{}", date_inscription.1.to_string()),
                    "date_debut_competition": format!("{}", date_competition.0.to_string()),
                    "date_fin_competition": format!("{}", date_competition.1.to_string())
                }}, None)
        .await
        .unwrap();

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

fn comparaison_dates(date1 : &DATE, date2 : &DATE) -> Ordering {
    if date1.annee > date2.annee {
        return Greater;
    } else if date1.annee == date2.annee {
        if date1.mois > date2.mois {
            return Greater;
        } else if date1.mois == date2.mois {
            if date1.jour > date2.jour {
                return Greater;
            }
            else { return Equal; }
        }
    }
    Less
}

#[test]
fn test_parsing_two_dates() {
    let date = "01/01/2021-02/02/2021";
    let date = parsing_two_dates(date);
    assert_eq!(comparaison_dates(&date.0, &date.1), Less);
    assert_eq!(comparaison_dates(&date.1, &date.0), Greater);
    assert_eq!(comparaison_dates(&date.0, &date.0), Equal);
}

fn verifications_dates(date_inscription : &(DATE, DATE), date_competition : &(DATE, DATE)) -> &'static str {
    match comparaison_dates(&date_inscription.0, &date_competition.0){
        Greater => {return "La date de début des inscriptions est supérieure à la date de fin du début de la compétition.";},
        _ => ()
    }

    match comparaison_dates(&date_inscription.1, &date_competition.1){
        Greater => {return "La date de fin des inscriptions est supérieure à la date de fin de la compétition.";},
        _ => ()
    }

    match comparaison_dates(&date_inscription.0, &date_inscription.1){
        Greater => {return "La date de début des inscriptions est supérieure à la date de fin des inscirptions.";},
        _ => ()
    }

    match comparaison_dates(&date_competition.0, &date_competition.1){
        Greater => {return "La date de début de la compétition est supérieure à la date de fin de la compétition.";},
        _ => ()
    }

    "ok"
}

fn verification_chevauchement_edition(date : &DATE, client : &MongoClient) {
    //todo
}

fn verification_existance_date(date : &DATE) -> Result< String, String> {
    if date.annee <= 2021 { return Err("Attention l'année saisie est passée !".to_string()) }
    match date.mois{
        2 => {
            if ((date.annee %4 == 0) && (date.annee %100 != 0)) || (date.annee % 400 == 0) {
                if (date.jour <= 29) && (date.jour >= 1){ return Ok(date.to_string())}
            }
            else if(date.jour <= 28) && (date.jour >= 1){ return Ok(date.to_string())}
            return Err("La date n'est pas dans le mois. Attention aux années bisextilles !".to_string())
        },
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            if (date.jour <= 31) && (date.jour >= 1){
                return Ok(date.to_string());
            }
            return Err("La date n'est pas dans le mois !".to_string());
        },
        4 | 6 | 9 | 11 => {
            if (date.jour <= 30) && (date.jour >= 1){
                return Ok(date.to_string());
            }
            return Err("La date n'est pas dans le mois !".to_string());
        },

        _ => Err("Le mois n'existe pas !".to_string())
    }
}

#[test]
fn test_verification_existance_date(){
    let date = parse_one_date(&"02/31/2022");
    assert!(verification_existance_date(&date).is_err() );
}

#[test]
fn test_verification_existance_date2(){
    let date = parse_one_date(&"25/02/2022");
    assert_eq!(verification_existance_date(&date).unwrap(), "25-02-2022");
}

async fn send_error (mci : &ModalSubmitInteraction, ctx : &Context, erreur : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Erreur !")
                        .description(erreur)
                        .color(0xff0000)
                        .to_owned()
                )
            })
    })
        .await
        .unwrap();
}