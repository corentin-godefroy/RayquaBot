use std::str::FromStr;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::id::RoleId;
use crate::commands::common_functions::send_error_from_command;
use crate::commands::constants::*;
use tokio::join;

pub async fn print_versions_setup(ctx: &Context){
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(PRINT_VERSIONS)
            .description("Affiche les versions autorisées pour l'édition")

    })
        .await;
}

pub async fn print_versions_reactor(command: &ApplicationCommandInteraction, ctx: &Context, client: &Client) {
    let com = &command.clone();
    
    let user_id = com.user.id.to_string();

    let editions = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                ORGANIZER: &user_id.as_str(),
                GUILD_ID: &command.guild_id.unwrap().0.to_string()
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
        let msg = format!("Aucune édition n'est actuellement affichable.");
        send_error_from_command(&com, &ctx, &msg).await;
        return;
    }
    
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(PRINT_VERSIONS_MODAL)
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

pub async fn print_versions(mci: &MessageComponentInteraction, ctx: &Context, client: &Client) {
    let guild_id = mci.guild_id.unwrap().0.to_string();
    let edition_name = mci.data.values.get(0).unwrap();

    let collection: Collection<Document> = client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
    
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