use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::UpdateOptions;
use serenity::client::Context;


use serenity::model::application::command::{Command};
use serenity::model::application::command::CommandOptionType::{SubCommand};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::command::CommandOptionType::{Integer};
use tokio::join;
use crate::commands::common_functions::{get_player_editions, get_versions_list_tuple};
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
    let com = command.clone();

    if command.guild_id.is_some() {
        let message = command.user.dm(&ctx.http, |m| {
            m.content("This command must be used here, in private message.")
        });

        let response = command.defer(&ctx.http);

        join!(message, response);

        command.delete_original_interaction_response(&ctx.http)
            .await
            .expect("Failed to send interaction response");
        return;
    }

    get_player_editions(ctx, &command, client, VERSION_PLAYER_MODAL).await;
}

fn interdiction_to_emote<'a>(edition: &'a Document, field: &'a str) -> &'a str {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction{
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
        PLAYER_ID: mci.user.id.0.to_string(),
        VERIFIED: false
    }, None).await.unwrap().unwrap();

    options.remove(0);
    for option in options{
        let param: Vec<&str> = option.split('-').collect();
        let version: &str = param[0];

        let filter = doc!{
            EDITION_NAME: &edition_name,
            GUILD_ID: &guild_id,
            PLAYER_ID: mci.user.id.0.to_string(),
            VERIFIED: false
        };

        let value = param[1].parse::<i32>().unwrap();

        let modif = doc! {
            "$set": { version : value}
        };

        collection.update_one(filter, modif, None).await.unwrap();
    }

    let filter = doc!{
        EDITION_NAME: &edition_name,
        GUILD_ID: &guild_id,
        PLAYER_ID: mci.user.id.0.to_string(),
        VERIFIED: false
    };

    let edition = collection.find_one(filter, None).await.unwrap().unwrap();

    let message_id = doc.get(MESSAGE_ID).unwrap().to_string().split("\"").collect::<String>().parse::<u64>().unwrap();

    let versions = get_versions_list_tuple();

    ctx.http.get_message(mci.channel_id.0, message_id).await.unwrap().edit(&ctx.http, |message|{
        message.embed(|embed| {
            embed.colour(LIGHT_BLUE_COLOR)
                .title(format!("Versions autorisées pour l'édition {}", edition_name));
            if versions.len() %2 == 1 {
                for i in (0.. versions.len() - 1).step_by(2) {
                    embed.field(format!("{} : \n{}\n--------------------------", versions[i].1, interdiction_to_emote(&edition, versions[i].0)),
                                format!("**{} : \n{}\n--------------------------**", versions[i + 1].1, interdiction_to_emote(&edition, versions[i + 1].0)), true);
                }
                embed.field(format!("{} : \n{}\n--------------------------", versions[versions.len() - 1].1, interdiction_to_emote(&edition, versions[versions.len() - 1].0)), "".to_string(), true);
            }
            else {
                for i in (0.. versions.len() - 1).step_by(2) {
                    embed.field(format!("{} : \n{}\n--------------------------", versions[i].1, interdiction_to_emote(&edition, versions[i].0)),
                                format!("**{} : \n{}\n--------------------------**", versions[i + 1].1, interdiction_to_emote(&edition, versions[i + 1].0)),true);
                }
            }
            embed
        })
    }).await.unwrap();

    mci.message.delete(&ctx.http).await.unwrap();
}