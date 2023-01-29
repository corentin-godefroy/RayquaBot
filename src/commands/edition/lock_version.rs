use std::ops::{Add, Sub};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::TimeZone;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::command::CommandOptionType::{SubCommand, SubCommandGroup};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::command::CommandOptionType::Boolean;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use crate::commands::common_functions::{send_error_from_command, send_error_from_component, send_error_from_modal};
use crate::commands::constants::*;

pub async fn lock_version_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name(LOCK_VERSION).description("Bloque une version poour l'édition choisie")
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
            
    })
        .await.expect("Creation of lock_version failed");
}

pub async fn lock_version_reactor(command: &ApplicationCommandInteraction, ctx: &Context, client: &Client) {
    let com = &command.clone();
    
    let user_id = com.user.id.to_string();
    
    let editions = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                ORGANIZER: &user_id.as_str(),
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
        , None).await.unwrap().collect::<Vec<_>>().await;
    
    let mut opt = Vec::new();
    
    for edition in editions {
        let edition = edition.unwrap().get(EDITION_NAME).unwrap().as_str().unwrap().to_string();
        opt.push(edition);
    }
    
    if opt.is_empty() {
        let msg = format!("Aucune édition n'est actuellement modifiable.");
        send_error_from_command(&com, &ctx, &msg).await;
        return;
    }
    
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(LOCK_VERSION_MODAL)
                                .placeholder("Choisis une édition")
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