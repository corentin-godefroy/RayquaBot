use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::{CreateEmbed};
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use chrono;
use chrono::{NaiveDate};

use serenity::model::application::component::InputTextStyle;
use serenity::model::id::GuildId;
use serenity::model::Permissions;

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
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("new_edition")
            .description("Créé une nouvelle édition.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await;
}

pub async unsafe fn new_edition_reactor(command : &ApplicationCommandInteraction, context : &Context) {
    let ctx = context;
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
                    .custom_id("new_edition_modal")
            )
    })
        .await
        .expect("Failed to send interaction response");
}

pub async fn prompt_edition_modal(client : &MongoClient, mci : ModalSubmitInteraction, ctx : serenity::client::Context) {
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


    let timestamp_debut_inscription = get_timestamp_from_date(&date_inscription.0);
    resolver_timestamp(&timestamp_debut_inscription, &mci, &ctx).await;
    let timestamp_fin_inscription = get_timestamp_from_date(&date_inscription.1);
    resolver_timestamp(&timestamp_fin_inscription, &mci, &ctx).await;
    let timestamp_debut_competition = get_timestamp_from_date(&date_competition.0);
    resolver_timestamp(&timestamp_debut_competition, &mci, &ctx).await;
    let timestamp_fin_competition = get_timestamp_from_date(&date_competition.1);
    resolver_timestamp(&timestamp_fin_competition, &mci, &ctx).await;

    let resultat = verification_chevauchement_edition(&timestamp_debut_inscription.as_ref().unwrap(), &timestamp_fin_competition.as_ref().unwrap(), &client, &guild, &ctx, &mci).await;

    match resultat {
        Ok(_) => {
            let collection =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
            let doc = doc! {
                ORGANISATEUR: organisateur,
                NOM_EDITION : nom_competition.value.as_str(),
                GUILD_ID    : &guild.0.to_string(),
                DATE_DEBUT_INSCRIPTION : &timestamp_debut_inscription.unwrap(),
                DATE_FIN_INSCRIPTION   : &timestamp_fin_inscription.unwrap(),
                DATE_DEBUT_COMPETITION : &timestamp_debut_competition.unwrap(),
                DATE_FIN_COMPETITION   : &timestamp_fin_competition.unwrap()
            };
            collection.insert_one(doc, None).await.expect("Failed to insert document");

            mci.create_interaction_response(ctx, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        d.add_embed(
                            CreateEmbed::default()
                                .title("Nouvelle édition créée avec succès !")
                                .description(format!(
                                    "Les inscription démarerons le {} et se fermerons le {}.\n\
                                    La compétition commenceras de {} et se termineras le {}."
                                    , date_inscription.0.to_string(), date_inscription.1.to_string(), date_competition.0.to_string(), date_competition.1.to_string())).to_owned()
                                .color(GREEN_COLOR)
                                .to_owned()
                        )
                    })
            })
                .await
                .unwrap();
        }
        Err(_) => {}
    }
}

fn parsing_two_dates(date : &str) -> (DATE, DATE) {
    let dates : Vec<String> = date.split("-").map(|s| s.to_string()).collect();

    let date1 = dates.get(0).unwrap();
    let date1 = parse_one_date(date1);

    let date2= dates.get(1).unwrap();
    let date2 = parse_one_date(date2);
    (date1, date2)
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

async fn verification_chevauchement_edition(timestamp_debut : &i64, timestamp_fin : &i64, client : &MongoClient, guild_id : &GuildId, ctx : &Context, mci : &ModalSubmitInteraction) ->Result<(), String>{
    //todo FIXIT

    let querry = doc! {
        GUILD_ID: &guild_id.0.to_string(),
        "$nor": [
            doc! { DATE_DEBUT_INSCRIPTION: doc! {"$gt": timestamp_fin}},
            doc! { DATE_FIN_COMPETITION  : doc! {"$lt": timestamp_debut}}
        ]
    };

    let count = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).count_documents(querry,None).await.unwrap();

    if count > 0 {
        send_error(&mci, &ctx, "L'édition chevauche une édition existante !").await;
        return Err("L'édition chevauche une édition existante !".to_string());
    }
    return Ok(());
}

async fn send_error (mci : &ModalSubmitInteraction, ctx : &Context, err : &str) {
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

fn get_timestamp_from_date(date : &DATE) -> Result<i64, String> {
    match NaiveDate::from_ymd_opt(date.annee as i32, date.mois as u32, date.jour as u32){
        Some(date) => {
            let date = date.and_hms(0, 0, 0);
            let timestamp = date.timestamp();
            Ok(timestamp)
        },
        None => Err("Date invalide".to_string())
    }
}

async fn resolver_timestamp(t : &Result<i64, String>, mci : &ModalSubmitInteraction, ctx : &Context) {
    match t {
        Err(err) => {
            send_error( &mci, &ctx, err).await;
        },
        _ => ()
    }
}