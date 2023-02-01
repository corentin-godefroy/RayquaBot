use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::UpdateOptions;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::application::command::{Command};
use serenity::model::application::command::CommandOptionType::{SubCommand};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::command::CommandOptionType::Boolean;
use serenity::model::prelude::interaction::modal::ModalSubmitInteraction;
use crate::commands::common_functions::{send_error_from_command};
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
            
    })
        .await.expect("Creation of lock_version failed");
}

pub async fn version_setup_reactor(command: &ApplicationCommandInteraction, ctx: &Context, client: &Client) {
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
    
    let mut custom_id = "".to_owned();
    
    let options = command.data.options.get(0).unwrap().options.to_vec();
    
    for option in options {
        custom_id = custom_id + "-" + option.name.as_str() + "*" + option.value.unwrap().to_string().as_str()
    }
    
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(LOCK_VERSION_MODAL.to_owned() + custom_id.as_str())
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

pub async fn version_setup_end(mci: &MessageComponentInteraction, ctx: &Context, client: &Client){
    let mut options: Vec<&str> = mci.data.custom_id.split('-').collect();
    let guild_id = mci.guild_id.unwrap().0.to_string();
    let edition_name = mci.data.values.get(0).unwrap();
    
    let collection: mongodb::Collection<Document> =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
    
    options.remove(0);
    for option in options{
        let param: Vec<&str> = option.split('*').collect();
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
    
    mci.create_interaction_response(&ctx.http, |response|{
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.embed(|embed|{
                    embed.colour(LIGHT_BLUE_COLOR)
                    .title(format!("Versions autorisées pour l'édition {}", edition_name))
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_RED_GREEN_BLUE,               if edition.get(BDD_POKE_RED_GREEN_BLUE)              .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}), 
                               format!("**{} : \n{}**\n---------------------------",POKE_YELLOW,                       if edition.get(BDD_POKE_YELLOW)                      .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_GOLD_SILVER,                  if edition.get(BDD_POKE_GOLD_SILVER)                 .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_CRYSTAL,                      if edition.get(BDD_POKE_CRYSTAL)                     .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_RUBY_SAPPHIRE,                if edition.get(BDD_POKE_RUBY_SAPPHIRE)               .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_FIRERED_LEAFGREEN,            if edition.get(BDD_POKE_FIRERED_LEAFGREEN)           .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_EMERALD,                      if edition.get(BDD_POKE_EMERALD)                     .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_DIAMOND_PEARL,                if edition.get(BDD_POKE_DIAMOND_PEARL)               .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_PLATINUM,                     if edition.get(BDD_POKE_PLATINUM)                    .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_HEARTGOLD_SOULSILVER,         if edition.get(BDD_POKE_HEARTGOLD_SOULSILVER)        .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_BLACK_WHITE,                  if edition.get(BDD_POKE_BLACK_WHITE)                 .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_BLACK2_WHITE2,                if edition.get(BDD_POKE_BLACK2_WHITE2)               .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_X_Y,                          if edition.get(BDD_POKE_X_Y)                         .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_OMEGA_RUBY_ALPHA_SAPPHIRE,    if edition.get(BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)   .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_SUN_MOON,                     if edition.get(BDD_POKE_SUN_MOON)                    .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_ULTRASUN_ULTRAMOON,           if edition.get(BDD_POKE_ULTRASUN_ULTRAMOON)          .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_LETSGOPIKACHU_LETSGOEEVEE,    if edition.get(BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE)   .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_SWORD_SHIELD,                 if edition.get(BDD_POKE_SWORD_SHIELD)                .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_BRILLANTDIAMOND_SHININGPEARL, if edition.get(BDD_POKE_BRILLANTDIAMOND_SHININGPEARL).unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_LEGENDARCEUS,                 if edition.get(BDD_POKE_LEGENDARCEUS)                .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_SCARLET_VIOLET,               if edition.get(BDD_POKE_SCARLET_VIOLET)              .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------",POKE_DONJON_MYSTERE,               if edition.get(BDD_POKE_DONJON_MYSTERE)              .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_STADIUM_EU,                   if edition.get(BDD_POKE_STADIUM_EU)                  .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               format!("**{} : \n{}**\n---------------------------", POKE_STADIUM_JAP,                 if edition.get(BDD_POKE_STADIUM_JAP)                 .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_STADIUM_2,                    if edition.get(BDD_POKE_STADIUM_2)                   .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               "", true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_XD,                           if edition.get(BDD_POKE_XD)                          .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               "", true)
                        .field(format!("{} : \n{}\n---------------------------"    ,POKE_COLOSEUM,                     if edition.get(BDD_POKE_COLOSEUM)                    .unwrap().as_i32().unwrap() == 1 {"✅"} else {"❌"}),
                               "", true)
                })
            })
    }).await.unwrap();
    
    mci.delete_followup_message(&ctx.http, mci.message.id.0).await.unwrap();
}