use std::any::Any;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::UpdateOptions;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::json::prelude::from_str;
use serenity::model::application::command::{Command};
use serenity::model::application::command::CommandOptionType::{SubCommand};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::command::CommandOptionType::{Integer};
use serenity::model::prelude::interaction::InteractionType::ApplicationCommand;
use serenity::model::prelude::interaction::modal::ModalSubmitInteraction;
use crate::commands::common_functions::{send_error_from_command};
use crate::commands::constants::*;

pub async fn version_player_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name(VERSION_PLAYER_SETUP).description("Sers à indiquer les versions possédées")
            .create_option(|option|{
                option.name("gen_1")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_RED_GREEN_BLUE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_RED_GREEN_BLUE)

                    })

                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_YELLOW)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_YELLOW)
                    })

                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_EU)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_STADIUM_EU)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_JAP)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(BDD_POKE_STADIUM_JAP)
                    })
            })

            .create_option(|option|{
                option.name("gen_2")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_GOLD_SILVER)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_GOLD_SILVER)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_CRYSTAL)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_CRYSTAL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_2)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_STADIUM_2)
                    })
            })

            .create_option(|option|{
                option.name("gen_3")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_RUBY_SAPPHIRE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_RUBY_SAPPHIRE)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_EMERALD)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_EMERALD)
                    })

                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_FIRERED_LEAFGREEN)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_FIRERED_LEAFGREEN)
                    })

                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_COLOSEUM)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_COLOSEUM)
                    })

                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_XD)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_XD)
                    })
            })

            .create_option(|option|{
                option.name("gen_4")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_DIAMOND_PEARL)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_DIAMOND_PEARL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_PLATINUM)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_PLATINUM)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_HEARTGOLD_SOULSILVER)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_HEARTGOLD_SOULSILVER)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BLACK_WHITE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_BLACK_WHITE)
                    })
            })

            .create_option(|option|{
                option.name("gen_5")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BLACK2_WHITE2)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_BLACK2_WHITE2)
                    })
            })

            .create_option(|option|{
                option.name("gen_6")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_X_Y)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(BDD_POKE_X_Y)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)
                    })
            })

            .create_option(|option|{
                option.name("gen_7")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SUN_MOON)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_SUN_MOON)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_ULTRASUN_ULTRAMOON)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_ULTRASUN_ULTRAMOON)
                    })
                    .create_sub_option(|option|{
                        option.name(BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description("select generation")
                    })
            })

            .create_option(|option|{
                option.name("gen_8")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_DONJON_MYSTERE)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_DONJON_MYSTERE)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SWORD_SHIELD)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_SWORD_SHIELD)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BRILLANTDIAMOND_SHININGPEARL)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_BRILLANTDIAMOND_SHININGPEARL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_LEGENDARCEUS)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_LEGENDARCEUS)
                    })
            })

            .create_option(|option|{
                option.name("gen_9")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SCARLET_VIOLET)
                            .kind(Integer)
                            .add_int_choice("Non possédé (défaut)", 0)
                            .add_int_choice("Possédé sans Charme Chroma", 1)
                            .add_int_choice("Possédé avec Charme Chroma", 2)
                            .description(POKE_SCARLET_VIOLET)
                    })
            })
            .dm_permission(true)
    })
        .await.expect("Creation of version_player_setup failed");
}

pub async fn version_player_setup_reactor(command: &ApplicationCommandInteraction, ctx: &Context, client: &Client) {
    let mut com = command.clone();

    if com.guild_id.is_some(){
        command.create_interaction_response(&ctx.http, |response|{
            response.kind(ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.content("Cette commande ne peut être utilisée qu'en message privé")
                })
        })
            .await.expect("Failed to send message");
        return;
    }

    let collection: mongodb::Collection<Document> =  client.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION);

    let mut editions: Vec<Document> = collection.aggregate(
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

    let mut edition_names: Vec<(String, String)> = Vec::new();

    for edition in editions.iter(){
        edition_names.push((edition.get(EDITION_NAME).unwrap().as_str().unwrap().to_string(), edition.get(GUILD_ID).unwrap().as_str().unwrap().to_string()));
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

    let mut options = command.data.options.to_vec().get(0).unwrap().options.to_vec();



    for option in options{
        options_str = options_str + "*" + option.name.as_str() + "-" + option.value.as_ref().unwrap().as_i64().unwrap().to_string().as_str()
    }

    &command.create_interaction_response(&ctx.http, |response|{
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.content("Sélectionne l'édition à modifier")
                    .components(|components|{
                        components.create_action_row(|action_row|{
                            action_row.create_select_menu(|select_menu|{
                                select_menu.custom_id(VERSION_PLAYER_MODAL.to_string() + "-" + options_str.as_str())
                                    .placeholder("Sélectionne une édition")
                                    .options(|options|{
                                        for edition_name in edition_names.iter(){
                                            options.create_option(|option|{
                                                option.label(edition_name.0.to_string())
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

fn interdiction_to_emote<'a>(edition: &'a Document, field: &'a str) -> &'a str {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction{
        -1 => "INTERDIT",
         0 => "❌",
         1 => "✅",
         2 => "✅💫",
         _ => "Erreur"
    }
}

pub async fn version_player_setup_end(mci: &MessageComponentInteraction, ctx: &Context, client: &Client){
    let mut options: Vec<&str> = mci.data.custom_id.split('*').collect();
    let values = mci.data.values.clone().to_vec();

    let guild_id = values[0].split('-').collect::<Vec<&str>>()[1].to_string();
    let edition_name = values[0].split('-').collect::<Vec<&str>>()[0].to_string();

    let collection: mongodb::Collection<Document> =  client.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION);

    let doc = collection.find_one(doc!{
        EDITION_NAME: &edition_name,
        GUILD_ID: &guild_id,
        PLAYER_ID: mci.user.id.0.to_string()
    }, None).await.unwrap().unwrap();

    options.remove(0);
    for option in options{
        let param: Vec<&str> = option.split('-').collect();
        let version: &str = param[0];

        let filter = doc! {
            EDITION_NAME : &edition_name,
            GUILD_ID     : &guild_id
        };

        let mut value = param[1].parse::<i32>().unwrap();

        if doc.get(version).unwrap().as_i32().unwrap() == -1{
            mci.create_interaction_response(&ctx.http, |response|{
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message|{
                        message.content(format!("La version {} n'est pas autorisée pour l'édition {}", version, edition_name))
                    })
            })
                .await
                .expect("Failed to send message");
            continue;
        }

        let modif = doc! {
            "$set": { version : value}
        };

        collection.update_one(filter, modif, None).await.unwrap();
    }

    let filter = doc! {
        EDITION_NAME : &edition_name,
        GUILD_ID     : &guild_id
    };

    let edition = collection.find_one(filter, None).await.unwrap().unwrap();
    
    let message_id = from_str::<u64>(&doc.get(MESSAGE_ID).unwrap().to_string()).unwrap();

    mci.create_interaction_response (&ctx.http, |message|{
        message.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|response|{
               response.embed(|embed| {
                   embed.colour(LIGHT_BLUE_COLOR)
                       .title(format!("Versions autorisées pour l'édition {}", edition_name))
                       .field(format!("{} : \n{}\n---------------------------", POKE_RED_GREEN_BLUE, interdiction_to_emote(&edition, BDD_POKE_RED_GREEN_BLUE)),
                              format!("**{} : \n{}**\n---------------------------", POKE_YELLOW, interdiction_to_emote(&edition, BDD_POKE_YELLOW)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_GOLD_SILVER, interdiction_to_emote(&edition, BDD_POKE_GOLD_SILVER)),
                              format!("**{} : \n{}**\n---------------------------", POKE_CRYSTAL, interdiction_to_emote(&edition, BDD_POKE_CRYSTAL)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_RUBY_SAPPHIRE, interdiction_to_emote(&edition, BDD_POKE_RUBY_SAPPHIRE)),
                              format!("**{} : \n{}**\n---------------------------", POKE_FIRERED_LEAFGREEN, interdiction_to_emote(&edition, BDD_POKE_FIRERED_LEAFGREEN)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_EMERALD, interdiction_to_emote(&edition, BDD_POKE_EMERALD)),
                              format!("**{} : \n{}**\n---------------------------", POKE_DIAMOND_PEARL, interdiction_to_emote(&edition, BDD_POKE_DIAMOND_PEARL)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_PLATINUM, interdiction_to_emote(&edition, BDD_POKE_PLATINUM)),
                              format!("**{} : \n{}**\n---------------------------", POKE_HEARTGOLD_SOULSILVER, interdiction_to_emote(&edition, BDD_POKE_HEARTGOLD_SOULSILVER)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_BLACK_WHITE, interdiction_to_emote(&edition, BDD_POKE_BLACK_WHITE)),
                              format!("**{} : \n{}**\n---------------------------", POKE_BLACK2_WHITE2, interdiction_to_emote(&edition, BDD_POKE_BLACK2_WHITE2)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_X_Y, interdiction_to_emote(&edition, BDD_POKE_X_Y)),
                              format!("**{} : \n{}**\n---------------------------", POKE_OMEGA_RUBY_ALPHA_SAPPHIRE, interdiction_to_emote(&edition, BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_SUN_MOON, interdiction_to_emote(&edition, BDD_POKE_SUN_MOON)),
                              format!("**{} : \n{}**\n---------------------------", POKE_ULTRASUN_ULTRAMOON, interdiction_to_emote(&edition, BDD_POKE_ULTRASUN_ULTRAMOON)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_LETSGOPIKACHU_LETSGOEEVEE, interdiction_to_emote(&edition, BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE)),
                              format!("**{} : \n{}**\n---------------------------", POKE_SWORD_SHIELD, interdiction_to_emote(&edition, BDD_POKE_SWORD_SHIELD)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_BRILLANTDIAMOND_SHININGPEARL, interdiction_to_emote(&edition, BDD_POKE_BRILLANTDIAMOND_SHININGPEARL)),
                              format!("**{} : \n{}**\n---------------------------", POKE_LEGENDARCEUS, interdiction_to_emote(&edition, BDD_POKE_LEGENDARCEUS)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_SCARLET_VIOLET, interdiction_to_emote(&edition, BDD_POKE_SCARLET_VIOLET)),
                              format!("**{} : \n{}**\n---------------------------", POKE_DONJON_MYSTERE, interdiction_to_emote(&edition, BDD_POKE_DONJON_MYSTERE)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_STADIUM_EU, interdiction_to_emote(&edition, BDD_POKE_STADIUM_EU)),
                              format!("**{} : \n{}**\n---------------------------", POKE_STADIUM_JAP, interdiction_to_emote(&edition, BDD_POKE_STADIUM_JAP)), true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_STADIUM_2, interdiction_to_emote(&edition, BDD_POKE_STADIUM_2)),
                              "", true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_XD, interdiction_to_emote(&edition, BDD_POKE_XD)),
                              "", true)
                       .field(format!("{} : \n{}\n---------------------------", POKE_COLOSEUM, interdiction_to_emote(&edition, BDD_POKE_COLOSEUM)),
                              "", true)
               })
        })
    }).await.unwrap();

    mci.delete_followup_message(&ctx.http, mci.message.id.0).await.unwrap();
}