use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use mongodb::{Client as MongoClient, Client};
use mongodb::bson::Document;
use crate::doc;
use crate::commands::constants::*;
use serenity::futures::StreamExt;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;

pub async fn send_error_from_modal (mci : &ModalSubmitInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.embed(|embed| {
                    embed.title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                })
            })
    })
        .await
        .unwrap();
}


pub async fn send_error_from_command (mci : &ApplicationCommandInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.embed(|embed|{
                    embed.title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                })
            })
    })
        .await
        .unwrap();
}

pub async fn send_error_from_component (mci : &MessageComponentInteraction, ctx : &Context, err : &str) {
    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.embed(|embed|{
                    embed.title("Erreur !")
                        .description(err)
                        .color(RED_COLOR)
                })
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
            ORGANIZER: &id
        },
        None
    ).await.unwrap()
}

pub async fn get_editions_names(client : &MongoClient, id : &str) -> Vec<Result<mongodb::bson::Document, mongodb::error::Error>> {
    client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                ORGANIZER: id,
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

pub fn get_versions_list_tuple() -> Vec<(&'static str,&'static str)> {
    vec![
        (BDD_POKE_RED_GREEN_BLUE              , POKE_RED_GREEN_BLUE),
        (BDD_POKE_YELLOW                      , POKE_YELLOW),
        (BDD_POKE_GOLD_SILVER                 , POKE_GOLD_SILVER),
        (BDD_POKE_CRYSTAL                     , POKE_CRYSTAL),
        (BDD_POKE_RUBY_SAPPHIRE               , POKE_RUBY_SAPPHIRE),
        (BDD_POKE_FIRERED_LEAFGREEN           , POKE_FIRERED_LEAFGREEN),
        (BDD_POKE_EMERALD                     , POKE_EMERALD),
        (BDD_POKE_DIAMOND_PEARL               , POKE_DIAMOND_PEARL),
        (BDD_POKE_PLATINUM                    , POKE_PLATINUM),
        (BDD_POKE_HEARTGOLD_SOULSILVER        , POKE_HEARTGOLD_SOULSILVER),
        (BDD_POKE_BLACK_WHITE                 , POKE_BLACK_WHITE),
        (BDD_POKE_BLACK2_WHITE2               , POKE_BLACK2_WHITE2),
        (BDD_POKE_X_Y                         , POKE_X_Y),
        (BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE   , POKE_OMEGA_RUBY_ALPHA_SAPPHIRE),
        (BDD_POKE_SUN_MOON                    , POKE_SUN_MOON),
        (BDD_POKE_ULTRASUN_ULTRAMOON          , POKE_ULTRASUN_ULTRAMOON),
        (BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE   , POKE_LETSGOPIKACHU_LETSGOEEVEE),
        (BDD_POKE_SWORD_SHIELD                , POKE_SWORD_SHIELD),
        (BDD_POKE_BRILLANTDIAMOND_SHININGPEARL, POKE_BRILLANTDIAMOND_SHININGPEARL),
        (BDD_POKE_LEGENDARCEUS                , POKE_LEGENDARCEUS),
        (BDD_POKE_SCARLET_VIOLET              , POKE_SCARLET_VIOLET),
        (BDD_POKE_DONJON_MYSTERE              , POKE_DONJON_MYSTERE),
        (BDD_POKE_COLOSEUM                    , POKE_COLOSEUM),
        (BDD_POKE_STADIUM_EU                  , POKE_STADIUM_EU),
        (BDD_POKE_STADIUM_JAP                 , POKE_STADIUM_JAP),
        (BDD_POKE_STADIUM_2                   , POKE_STADIUM_2),
        (BDD_POKE_XD                          , POKE_XD),
    ]
}

pub async fn get_player_editions(ctx: &Context, command: &ApplicationCommandInteraction, mongo_client: &Client, custom_modal_id: &str){
    let collection: mongodb::Collection<Document> =  mongo_client.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION);

    let editions: Vec<Document> = collection.aggregate(
        vec![
            doc!{
                "$match": doc!{
                    PLAYER_ID: command.user.id.0.to_string(),
                    VERIFIED: false
                }
            },
            doc!{
                "$project": doc!{
                    EDITION_NAME: 1,
                    GUILD_ID: 1
                }
            }
        ],
        None
    )
        .await
        .expect("Failed to aggregate")
        .map(|result| result.expect("Failed to get result"))
        .collect()
        .await;

    let mut edition_names: Vec<(String, String, String)> = Vec::new();

    for edition in editions.iter(){
        edition_names.push((
            edition.get(EDITION_NAME).unwrap().as_str().unwrap().to_string(),
            edition.get(GUILD_ID).unwrap().as_str().unwrap().to_string(),
            ctx.http.get_guild(edition.get(GUILD_ID).unwrap().as_str().unwrap().parse().unwrap()).await.unwrap().name.to_string()
        ));
    }

    if edition_names.len() == 0{
        command.create_interaction_response(&ctx.http, |response|{
            response.kind(ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.content("Tu n'est inscrit à aucune compétition dont la validation est à faire.")
                })
        })
            .await.expect("Failed to send message");
        return;
    }

    let mut options_str = "".to_owned();

    let options = command.data.options.to_vec().get(0).unwrap().options.to_vec();

    for option in options{
        options_str = options_str + "*" + option.name.as_str() + "-" + option.value.as_ref().unwrap().as_i64().unwrap().to_string().as_str()
    }

    command.create_interaction_response(&ctx.http, |response|{
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.content("Sélectionne l'édition à mettre à jour")
                    .components(|components|{
                        components.create_action_row(|action_row|{
                            action_row.create_select_menu(|select_menu|{
                                select_menu.custom_id(custom_modal_id.to_string() + options_str.as_str())
                                    .placeholder("Sélectionne une édition")
                                    .options(|options|{
                                        for edition_name in edition_names.iter(){
                                            options.create_option(|option|{
                                                option.label(edition_name.0.to_string() + " | " + edition_name.2.as_str())
                                                    .value(edition_name.0.to_string() + "-" + edition_name.1.to_string().as_str())
                                            });
                                        }
                                        options
                                    })
                            })
                        })
                    })
            })
    })
        .await
        .expect("Failed to send message");
}

pub async fn get_player_editions_from_modal(ctx: &Context, command: &ModalSubmitInteraction, mongo_client: &Client, custom_modal_id: &str){
    let collection: mongodb::Collection<Document> =  mongo_client.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION);

    let editions: Vec<Document> = collection.aggregate(
        vec![
            doc!{
                "$match": doc!{
                    PLAYER_ID: command.user.id.0.to_string(),
                    VERIFIED: false
                }
            },
            doc!{
                "$project": doc!{
                    EDITION_NAME: 1,
                    GUILD_ID: 1
                }
            }
        ],
        None
    )
        .await
        .expect("Failed to aggregate")
        .map(|result| result.expect("Failed to get result"))
        .collect()
        .await;

    let mut edition_names: Vec<(String, String, String)> = Vec::new();

    for edition in editions.iter(){
        edition_names.push((
            edition.get(EDITION_NAME).unwrap().as_str().unwrap().to_string(),
            edition.get(GUILD_ID).unwrap().as_str().unwrap().to_string(),
            ctx.http.get_guild(edition.get(GUILD_ID).unwrap().as_str().unwrap().parse().unwrap()).await.unwrap().name.to_string()
        ));
    }

    if edition_names.len() == 0{
        command.create_interaction_response(&ctx.http, |response|{
            response.kind(ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.content("Tu n'est inscrit à aucune compétition dont la validation est à faire.")
                })
        })
            .await.expect("Failed to send message");
        return;
    }

    command.create_interaction_response(&ctx.http, |response|{
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.content("Sélectionne l'édition à mettre à jour")
                    .components(|components|{
                        components.create_action_row(|action_row|{
                            action_row.create_select_menu(|select_menu|{
                                select_menu.custom_id(custom_modal_id)
                                    .placeholder("Sélectionne une édition")
                                    .options(|options|{
                                        for edition_name in edition_names.iter(){
                                            options.create_option(|option|{
                                                option.label(edition_name.0.to_string() + " | " + edition_name.2.as_str())
                                                    .value(edition_name.0.to_string() + "-" + edition_name.1.to_string().as_str())
                                            });
                                        }
                                        options
                                    })
                            })
                        })
                    })
            })
    })
        .await
        .expect("Failed to send message");
}