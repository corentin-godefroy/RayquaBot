use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use mongodb::bson::{doc, from_bson};
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::forward_to_deserialize_any;
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::channel::MessageType;
use serenity::model::id::RoleId;
use serenity::model::Permissions;
use crate::commands::common_functions::send_error_from_command;
use crate::commands::constants::*;
use tokio::join;

pub async fn registration_setup(ctx: &Context){
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(REGISTRATION)
            .description("Sers à s'inscrire à une édition.")
    })
        .await;
}

pub async fn get_registration_reactor(mongo: &Client, aci: &ApplicationCommandInteraction, ctx: &Context){
    let filter = doc!{
        GUILD_ID: doc!{ 
            "$eq": &aci.guild_id.unwrap().0.to_string()
        }
    };
    
    let setup = mongo.database(RAYQUABOT_DB).collection::<String>(SERVER_COLLECTION).find(filter, None).await.unwrap();
    if setup.current().is_empty(){
        send_error_from_command(&aci, &ctx, "Le serveur n'est pas setup. Demande à un membre ayant les permissions admin de faire la commande **__/setup__**").await;
        return;
    }
    
    let registration_channel_id = setup.current().get(REGISTRATION_CHANNEL_ID).unwrap().unwrap().as_str().unwrap();
    if aci.channel_id.0.to_string() != registration_channel_id{
        let message = aci.member.as_ref().unwrap().user.dm(&ctx.http, |message|{
            message.add_embed(|embed|{
                embed.colour(RED_COLOR)
                    .title("Inscription")
                    .description(format!("Pour t'inscrire, il fait faire /{} dans le salon {}. Si tu ne vois pas ce salon et que tu n'est pas déjà inscrit, demande à un admin.", REGISTRATION, REGISTRATION_CHANNEL_NAME).as_str())
            })
        });
        
        let remove = aci.delete_original_interaction_response(&ctx.http);
        join!(message, remove);
        return;
    }
    
    let registred_role_id = RoleId::from_str(setup.current().get(REGISTERED_ROLE_ID).unwrap().unwrap().as_str().unwrap()).unwrap();
    if aci.member.as_ref().unwrap().roles.contains(&registred_role_id){
        send_error_from_command(&aci, &ctx, "Tu est déjà isncrit à la compétition !").await;
        return;
    }
    
    let filter = doc!{
        GUILD_ID: doc!{ 
            "$eq": &aci.guild_id.unwrap().0.to_string()
        }
    };
    
    let edition = mongo.database(RAYQUABOT_DB).collection::<String>(EDITIONS_COLLECTION).find(filter, None).await.unwrap();
    
    let start_registration = edition.current().get(INSCRIPTION_START_DATE).unwrap().unwrap().as_i64().unwrap() *1000;
    let end_registration = edition.current().get(INSCRIPTION_END_DATE).unwrap().unwrap().as_i64().unwrap() * 1000;
    let command_timestamp = (aci.id.0>>22) + 1420070400000;
    
    
    if !(command_timestamp > start_registration as u64 && command_timestamp < end_registration as u64){
        aci.create_interaction_response(&ctx.http, |intearction|{
            intearction.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.embed(|embed|{
                        embed.colour(RED_COLOR)
                            .title("Inscriptions closes")
                            .description("Aucune édition n'est actuellement en phase d'inscription.")
                    })
                })
        }).await.unwrap();
        return;
    }
    
    let edition_name = edition.current().get(EDITION_NAME).unwrap().unwrap().as_str().unwrap();
    
    aci.user.dm(&ctx.http, |dm|{
        dm.content(format!("Pour continuer ta vérification, ça se passe ici.\n\
        Voici un petit guide pour t'aider dans cette tache.\n\
        La commande **/{}** sers à indiquer **__TOUS__** les noms de dresseur utilisé dans les versions que tu possède. Inutile d'indiquer la version.\n\
        La commande **/{}** vas te permettre d'indiquer 1 par un les versions que tu possède ainsi que si tu a un charme chroma pour AU MOINS UNE des versions concernées.\n\
        Si tu as besoin d'aide ou que tu as un doute, n'hésite pas à demander aux Host/Admin sur le serveur concerné.\n\
        Voici le récap des infos :", ADD_NAMES, ADD_VERSION))
    }).await.unwrap();
    
    //TODO récup les infos de l'édition et setup en fonction des valeurs par défaut.
    
    let message = aci.user.dm(&ctx.http, |dm|{
        dm.embed(|embed|{
            embed.colour(LIGHT_BLUE_COLOR)
                .title(format!("Récap de l'inscription pour l'édition {}", edition_name))
                .description("Toutes les infos que tu auras indiqués se trouvent ici.")
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_RED_GREEN_BLUE),
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_YELLOW),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_GOLD_SILVER), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_CRYSTAL),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_RUBY_SAPPHIRE), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_FIRERED_LEAFGREEN),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_EMERALD), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_DIAMOND_PEARL),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_PLATINUM), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_HEARTGOLD_SOULSILVER),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_BLACK_WHITE), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_BLACK2_WHITE2),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_X_Y), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_OMEGA_RUBY_ALPHA_SAPPHIRE),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_SUN_MOON), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_ULTRASUN_ULTRAMOON),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_LETSGOPIKACHU_LETSGOEEVEE), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_SWORD_SHIELD),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_BRILLANTDIAMOND_SHININGPEARL), 
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_LEGENDARCEUS),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_SCARLET_VIOLET),
                       format!("**{} : \nNon possédé**\n---------------------------",POKE_DONJON_MYSTERE),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_STADIUM_EU),
                       format!("**{} : \nNon possédé**\n---------------------------", POKE_STADIUM_JAP),true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_STADIUM_2),
                       "",true)
                .field(format!("{} : \nNon possédé\n---------------------------",POKE_XD),
                       "",true)
                .field(format!("{} : \nNon possédé\n---------------------------", POKE_COLOSEUM),
                "", true)
                .footer(|pied|{
                    pied.text("noms de dresseur : ")
                })
            })
            .components(|component|{
                component.create_action_row(|action_row|{
                    action_row.create_button(|button|{
                        button.custom_id(VALIDATE)
                            .style(ButtonStyle::Danger)
                            .label("Valider définitivement")
                    })
                })
            })
    }).await.unwrap();
    
    let player = doc!{
        PLAYER_ID: aci.user.id.0.to_string(),
        EDITION_NAME: edition.current().get(EDITION_NAME).unwrap().unwrap().as_str().unwrap(),
        GUILD_ID: aci.guild_id.unwrap().0.to_string(),
        TEAM: None::<String>, //id du role de la team
        VERIFIED: false, //booléen pour déterminer si le joueur a été validé ou pas encore.
        MESSAGE_ID: message.id.0.to_string(),
        TRAINER_NAME: "",
        MORE_INFO: "",
        BDD_POKE_RED_GREEN_BLUE              : 0,
        BDD_POKE_YELLOW                      : 0,
        BDD_POKE_GOLD_SILVER                 : 0,
        BDD_POKE_CRYSTAL                     : 0,
        BDD_POKE_RUBY_SAPPHIRE               : 0,
        BDD_POKE_FIRERED_LEAFGREEN           : 0,
        BDD_POKE_EMERALD                     : 0,
        BDD_POKE_DIAMOND_PEARL               : 0,
        BDD_POKE_PLATINUM                    : 0,
        BDD_POKE_HEARTGOLD_SOULSILVER        : 0,
        BDD_POKE_BLACK_WHITE                 : 0,
        BDD_POKE_BLACK2_WHITE2               : 0,
        BDD_POKE_X_Y                         : 0,
        BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE   : 0,
        BDD_POKE_SUN_MOON                    : 0,
        BDD_POKE_ULTRASUN_ULTRAMOON          : 0,
        BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE   : 0,
        BDD_POKE_SWORD_SHIELD                : 0,
        BDD_POKE_BRILLANTDIAMOND_SHININGPEARL: 0,
        BDD_POKE_LEGENDARCEUS                : 0,
        BDD_POKE_SCARLET_VIOLET              : 0,
        BDD_POKE_DONJON_MYSTERE              : 0,
        BDD_POKE_COLOSEUM                    : 0,
        BDD_POKE_STADIUM_EU                  : 0,
        BDD_POKE_STADIUM_JAP                 : 0,
        BDD_POKE_STADIUM_2                   : 0,
        BDD_POKE_XD                          : 0
    };
    
    mongo.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION).insert_one(player, None).await.expect("L'insertion d'un nouveau joueur à échoué.");
    
    let mut member = aci.member.clone().unwrap();
    
    member.add_role(&ctx.http, registred_role_id).await.unwrap();
    
    let name = edition.current().get(EDITION_NAME).unwrap().unwrap().as_str().unwrap();
    
    aci.create_interaction_response(&ctx.http, |response|{
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|{
                message.embed(|embed|{
                    embed.colour(GREEN_COLOR)
                        .title("Inscription validée")
                        .description(format!("{} s'est inscrit avec succès à l'édition **{}**", aci.user.name, name))
                })
            })
    }).await.unwrap();
}