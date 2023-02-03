use std::str::FromStr;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::id::RoleId;
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

fn interdiction_to_string<'a>(edition: &'a Document, field: &'a str) -> &'a str {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction{
        0 => VERSION_INTERDITE_STR,
        _ => "❌"
    }
}

fn interdiction_to_value<'a>(edition: &'a Document, field: &'a str) -> i32 {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction{
        0 => VERSION_INTERDITE_VALUE,
        _ => NON_POSSEDE_VALUE
    }
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

    let timestamp: i64 = (((aci.id.0 >> 22) + 1420070400000) / 1000 as u64) as i64;
    
    let filter = doc!{
        GUILD_ID: doc!{ 
            "$eq": &aci.guild_id.unwrap().0.to_string()
        },
        INSCRIPTION_START_DATE: doc!{
            "$lte": &timestamp
        },
        INSCRIPTION_END_DATE: doc!{
            "$gte": &timestamp
        }
    };
    
    let edition = mongo.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).find_one(filter, None).await.unwrap().unwrap();
    
    let start_registration = edition.get(INSCRIPTION_START_DATE).unwrap().as_i64().unwrap();
    let end_registration = edition.get(INSCRIPTION_END_DATE).unwrap().as_i64().unwrap();

    if !(timestamp > start_registration && timestamp < end_registration){
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
    
    let edition_name = edition.get(EDITION_NAME).unwrap().as_str().unwrap();
    
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
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_RED_GREEN_BLUE, interdiction_to_string(&edition, BDD_POKE_RED_GREEN_BLUE)),
                       format!("**{} : \n{}\n---------------------------**",POKE_YELLOW, interdiction_to_string(&edition, BDD_POKE_YELLOW)), true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_GOLD_SILVER, interdiction_to_string(&edition, BDD_POKE_GOLD_SILVER)),
                       format!("**{} : \n{}\n---------------------------**",POKE_CRYSTAL, interdiction_to_string(&edition, BDD_POKE_CRYSTAL)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_RUBY_SAPPHIRE, interdiction_to_string(&edition, BDD_POKE_RUBY_SAPPHIRE)),
                       format!("**{} : \n{}\n---------------------------**",POKE_FIRERED_LEAFGREEN, interdiction_to_string(&edition, BDD_POKE_FIRERED_LEAFGREEN)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_EMERALD, interdiction_to_string(&edition, BDD_POKE_EMERALD)),
                       format!("**{} : \n{}\n---------------------------**",POKE_DIAMOND_PEARL, interdiction_to_string(&edition, BDD_POKE_DIAMOND_PEARL)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_PLATINUM, interdiction_to_string(&edition, BDD_POKE_PLATINUM)),
                       format!("**{} : \n{}\n---------------------------**",POKE_HEARTGOLD_SOULSILVER, interdiction_to_string(&edition, BDD_POKE_HEARTGOLD_SOULSILVER)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_BLACK_WHITE, interdiction_to_string(&edition, BDD_POKE_BLACK_WHITE)),
                       format!("**{} : \n{}\n---------------------------**",POKE_BLACK2_WHITE2, interdiction_to_string(&edition, BDD_POKE_BLACK2_WHITE2)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_X_Y, interdiction_to_string(&edition, BDD_POKE_X_Y)),
                       format!("**{} : \n{}\n---------------------------**",POKE_OMEGA_RUBY_ALPHA_SAPPHIRE, interdiction_to_string(&edition, BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_SUN_MOON, interdiction_to_string(&edition, BDD_POKE_SUN_MOON)),
                       format!("**{} : \n{}\n---------------------------**",POKE_ULTRASUN_ULTRAMOON, interdiction_to_string(&edition, BDD_POKE_ULTRASUN_ULTRAMOON)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_LETSGOPIKACHU_LETSGOEEVEE, interdiction_to_string(&edition, BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE)),
                       format!("**{} : \n{}\n---------------------------**",POKE_SWORD_SHIELD, interdiction_to_string(&edition, BDD_POKE_SWORD_SHIELD)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_BRILLANTDIAMOND_SHININGPEARL, interdiction_to_string(&edition, BDD_POKE_BRILLANTDIAMOND_SHININGPEARL)),
                       format!("**{} : \n{}\n---------------------------**",POKE_LEGENDARCEUS, interdiction_to_string(&edition, BDD_POKE_LEGENDARCEUS)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_SCARLET_VIOLET, interdiction_to_string(&edition, BDD_POKE_SCARLET_VIOLET)),
                       format!("**{} : \n{}\n---------------------------**",POKE_DONJON_MYSTERE, interdiction_to_string(&edition, BDD_POKE_DONJON_MYSTERE)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_STADIUM_EU, interdiction_to_string(&edition, BDD_POKE_STADIUM_EU)),
                       format!("**{} : \n{}\n---------------------------**",POKE_STADIUM_JAP, interdiction_to_string(&edition, BDD_POKE_STADIUM_JAP)),true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_STADIUM_2, interdiction_to_string(&edition, BDD_POKE_STADIUM_2)),
                       "",true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_XD, interdiction_to_string(&edition, BDD_POKE_XD)),
                       "",true)
                .field(format!(  "{} : \n{}\n---------------------------",  POKE_COLOSEUM, interdiction_to_string(&edition, BDD_POKE_COLOSEUM)),
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
        EDITION_NAME: edition.get(EDITION_NAME).unwrap().as_str().unwrap(),
        GUILD_ID: aci.guild_id.unwrap().0.to_string(),
        TEAM: None::<String>, //id du role de la team
        VERIFIED: false, //booléen pour déterminer si le joueur a été validé ou pas encore.
        MESSAGE_ID: message.id.0.to_string(),
        TRAINER_NAME: "",
        MORE_INFO: "",
        BDD_POKE_RED_GREEN_BLUE              : interdiction_to_value(&edition, BDD_POKE_RED_GREEN_BLUE),
        BDD_POKE_YELLOW                      : interdiction_to_value(&edition, BDD_POKE_YELLOW),
        BDD_POKE_GOLD_SILVER                 : interdiction_to_value(&edition, BDD_POKE_GOLD_SILVER),
        BDD_POKE_CRYSTAL                     : interdiction_to_value(&edition, BDD_POKE_CRYSTAL),
        BDD_POKE_RUBY_SAPPHIRE               : interdiction_to_value(&edition, BDD_POKE_RUBY_SAPPHIRE),
        BDD_POKE_FIRERED_LEAFGREEN           : interdiction_to_value(&edition, BDD_POKE_FIRERED_LEAFGREEN),
        BDD_POKE_EMERALD                     : interdiction_to_value(&edition, BDD_POKE_EMERALD),
        BDD_POKE_DIAMOND_PEARL               : interdiction_to_value(&edition, BDD_POKE_DIAMOND_PEARL),
        BDD_POKE_PLATINUM                    : interdiction_to_value(&edition, BDD_POKE_PLATINUM),
        BDD_POKE_HEARTGOLD_SOULSILVER        : interdiction_to_value(&edition, BDD_POKE_HEARTGOLD_SOULSILVER),
        BDD_POKE_BLACK_WHITE                 : interdiction_to_value(&edition, BDD_POKE_BLACK_WHITE),
        BDD_POKE_BLACK2_WHITE2               : interdiction_to_value(&edition, BDD_POKE_BLACK2_WHITE2),
        BDD_POKE_X_Y                         : interdiction_to_value(&edition, BDD_POKE_X_Y),
        BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE   : interdiction_to_value(&edition, BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE),
        BDD_POKE_SUN_MOON                    : interdiction_to_value(&edition, BDD_POKE_SUN_MOON),
        BDD_POKE_ULTRASUN_ULTRAMOON          : interdiction_to_value(&edition, BDD_POKE_ULTRASUN_ULTRAMOON),
        BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE   : interdiction_to_value(&edition, BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE),
        BDD_POKE_SWORD_SHIELD                : interdiction_to_value(&edition, BDD_POKE_SWORD_SHIELD),
        BDD_POKE_BRILLANTDIAMOND_SHININGPEARL: interdiction_to_value(&edition, BDD_POKE_BRILLANTDIAMOND_SHININGPEARL),
        BDD_POKE_LEGENDARCEUS                : interdiction_to_value(&edition, BDD_POKE_LEGENDARCEUS),
        BDD_POKE_SCARLET_VIOLET              : interdiction_to_value(&edition, BDD_POKE_SCARLET_VIOLET),
        BDD_POKE_DONJON_MYSTERE              : interdiction_to_value(&edition, BDD_POKE_DONJON_MYSTERE),
        BDD_POKE_COLOSEUM                    : interdiction_to_value(&edition, BDD_POKE_COLOSEUM),
        BDD_POKE_STADIUM_EU                  : interdiction_to_value(&edition, BDD_POKE_STADIUM_EU),
        BDD_POKE_STADIUM_JAP                 : interdiction_to_value(&edition, BDD_POKE_STADIUM_JAP),
        BDD_POKE_STADIUM_2                   : interdiction_to_value(&edition, BDD_POKE_STADIUM_2),
        BDD_POKE_XD                          : interdiction_to_value(&edition, BDD_POKE_XD),
    };
    
    mongo.database(RAYQUABOT_DB).collection(PLAYER_COLLECTION).insert_one(player, None).await.expect("L'insertion d'un nouveau joueur à échoué.");
    
    let mut member = aci.member.clone().unwrap();
    
    member.add_role(&ctx.http, registred_role_id).await.unwrap();
    
    let name = edition.get(EDITION_NAME).unwrap().as_str().unwrap();
    
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