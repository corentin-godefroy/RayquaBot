use mongodb::bson::{doc, Document};
use mongodb::Client;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::application::command::{Command};
use serenity::model::application::command::CommandOptionType::{SubCommand};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType::Boolean;
use crate::commands::common_functions::get_versions_list_tuple;
use crate::commands::constants::*;

pub async fn version_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name(VERSION_SETUP).description("Bloque une version poour l'édition choisie")
            .create_option(|option|{
                option.name("gen_1")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_RED_GREEN_BLUE)
                            .kind(Boolean)
                            .description(POKE_RED_GREEN_BLUE)
                    })
    
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_YELLOW)
                            .kind(Boolean)
                            .description(POKE_YELLOW)
                    })
    
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_EU)
                            .kind(Boolean)
                            .description(POKE_STADIUM_EU)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_JAP)
                            .kind(Boolean)
                            .description(BDD_POKE_STADIUM_JAP)
                    })
            })
            
            .create_option(|option|{
                option.name("gen_2")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_GOLD_SILVER)
                            .kind(Boolean)
                            .description(POKE_GOLD_SILVER)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_CRYSTAL)
                            .kind(Boolean)
                            .description(POKE_CRYSTAL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_STADIUM_2)
                            .kind(Boolean)
                            .description(POKE_STADIUM_2)
                    })
            })
            
            .create_option(|option|{
                option.name("gen_3")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_RUBY_SAPPHIRE)
                            .kind(Boolean)
                            .description(POKE_RUBY_SAPPHIRE)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_EMERALD)
                            .kind(Boolean)
                            .description(POKE_EMERALD)
                    })
    
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_FIRERED_LEAFGREEN)
                            .kind(Boolean)
                            .description(POKE_FIRERED_LEAFGREEN)
                    })
    
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_COLOSEUM)
                            .kind(Boolean)
                            .description(POKE_COLOSEUM)
                    })
    
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_XD)
                            .kind(Boolean)
                            .description(POKE_XD)
                    })
            })
            
            .create_option(|option|{
                option.name("gen_4")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_DIAMOND_PEARL)
                            .kind(Boolean)
                            .description(POKE_DIAMOND_PEARL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_PLATINUM)
                            .kind(Boolean)
                            .description(POKE_PLATINUM)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_HEARTGOLD_SOULSILVER)
                            .kind(Boolean)
                            .description(POKE_HEARTGOLD_SOULSILVER)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BLACK_WHITE)
                            .kind(Boolean)
                            .description(POKE_BLACK_WHITE)
                    })
            })
    
            .create_option(|option|{
                option.name("gen_5")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BLACK2_WHITE2)
                            .kind(Boolean)
                            .description(POKE_BLACK2_WHITE2)
                    })
            })
    
            .create_option(|option|{
                option.name("gen_6")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_X_Y)
                            .kind(Boolean)
                            .description(BDD_POKE_X_Y)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)
                            .kind(Boolean)
                            .description(POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)
                    })
            })
    
            .create_option(|option|{
                option.name("gen_7")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SUN_MOON)
                            .kind(Boolean)
                            .description(POKE_SUN_MOON)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_ULTRASUN_ULTRAMOON)
                            .kind(Boolean)
                            .description(POKE_ULTRASUN_ULTRAMOON)
                    })
                    .create_sub_option(|option|{
                        option.name(BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE)
                            .kind(Boolean)
                            .description("select generation")
                    })
            })

            .create_option(|option|{
                option.name("gen_8")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_DONJON_MYSTERE)
                            .kind(Boolean)
                            .description(POKE_DONJON_MYSTERE)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SWORD_SHIELD)
                            .kind(Boolean)
                            .description(POKE_SWORD_SHIELD)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_BRILLANTDIAMOND_SHININGPEARL)
                            .kind(Boolean)
                            .description(POKE_BRILLANTDIAMOND_SHININGPEARL)
                    })
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_LEGENDARCEUS)
                            .kind(Boolean)
                            .description(POKE_LEGENDARCEUS)
                    })
            })
    
            .create_option(|option|{
                option.name("gen_9")
                    .kind(SubCommand)
                    .description("select generation")
                    .create_sub_option(|sub_option|{
                        sub_option.name(BDD_POKE_SCARLET_VIOLET)
                            .kind(Boolean)
                            .description(POKE_SCARLET_VIOLET)
                    })
            })
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await.expect("Creation of lock_version failed");
}

pub async fn version_setup_reactor(command: &ApplicationCommandInteraction, ctx: &Context, client: &Client) {
    let com = command.clone();

    if com.guild_id.is_none(){
        command.create_interaction_response(&ctx.http, |response|{
            response.kind(ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.content("Cette commande ne peut être utilisée que dans un serveur")
                })
        })
            .await.expect("Failed to send message");
        return;
    }



    if com.member.unwrap().permissions.unwrap().contains(Permissions::ADMINISTRATOR) == false{
        command.create_interaction_response(&ctx.http, |response|{
            response.kind(ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.content("Tu n'as pas les permissions nécessaires pour utiliser cette commande")
                })
        })
            .await.expect("Failed to send message");
        return;
    }

    let guild_id = &command.guild_id.unwrap().0.to_string();

    let collection: mongodb::Collection<Document> =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);

    let editions: Vec<Document> = collection.aggregate(
        vec![
            doc!{
                "$match": doc!{
                    GUILD_ID: guild_id
                }
            },
            doc!{
                "$sort": doc!{
                    EDITION_NAME: 1
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

    let mut edition_names: Vec<String> = Vec::new();

    for edition in editions.iter(){
        edition_names.push(edition.get(EDITION_NAME).unwrap().as_str().unwrap().to_string());
    }

    let mut options_str = "".to_owned();

    let options = command.data.options.to_vec().get(0).unwrap().options.to_vec();

    for option in options{
        options_str = options_str + "*" + option.name.as_str() + "-" + option.value.as_ref().unwrap().as_bool().unwrap().to_string().as_str()
    }

    command.create_interaction_response(&ctx.http, |response|{
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.content("Sélectionne l'édition à modifier")
                    .components(|components|{
                        components.create_action_row(|action_row|{
                            action_row.create_select_menu(|select_menu|{
                                select_menu.custom_id(LOCK_VERSION_MODAL.to_string() + options_str.as_str())
                                    .placeholder("Sélectionne une édition")
                                    .options(|options|{
                                        for edition_name in edition_names.iter(){
                                            options.create_option(|option|{
                                                option.label(edition_name)
                                                    .value(edition_name.to_string())
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
        0 => "❌",
        1 => "✅",
        _ => "Erreur"
    }
}

pub async fn version_setup_end(mci: &MessageComponentInteraction, ctx: &Context, client: &Client){
    let mut options: Vec<&str> = mci.data.custom_id.split('*').collect();

    let guild_id = &mci.guild_id.unwrap().0.to_string();

    let edition_name = mci.data.values.get(0).unwrap();
    
    let collection: mongodb::Collection<Document> =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
    
    options.remove(0);
    for option in options{
        let param: Vec<&str> = option.split('-').collect();
        let version: &str = param[0];
    
        let filter = doc! {
            EDITION_NAME : edition_name,
            GUILD_ID     : &guild_id
        };
        
        let mut value: u32 = 0;
        if param[1].parse::<bool>().unwrap() {
            value = 1;
        }
        
        let modif = doc! {
            "$set": { version : value}
        };
        
        collection.update_one(filter, modif, None).await.unwrap();
    }
    
    let filter = doc! {
        EDITION_NAME : edition_name,
        GUILD_ID     : &guild_id
    };
    
    let edition = collection.find_one(filter, None).await.unwrap().unwrap();

    let versions = get_versions_list_tuple();
    
    mci.create_interaction_response(&ctx.http, |response|{
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|{
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
            })
    }).await.unwrap();
    
    mci.delete_followup_message(&ctx.http, mci.message.id.0).await.unwrap();
}