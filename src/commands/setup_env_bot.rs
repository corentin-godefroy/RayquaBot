use std::collections::HashMap;
use mongodb::{Client};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{InteractionResponseType};
use serenity::model::channel::ChannelType::{Category, Text, Voice, Forum};
use serenity::model::guild::{PartialGuild, Role};
use serenity::model::id::{RoleId};
use serenity::model::channel::{GuildChannel, PermissionOverwrite};
use serenity::model::permissions::Permissions;
use serenity::model::prelude::PermissionOverwriteType;
use tokio::join;
use crate::commands::constants::{EVERYONE_ROLE_NAME, FAQ_CHANNEL_NAME, REGISTERED_ROLE_ID, BOT_RETURN_CHANNEL_NAME};
use crate::constants::*;
use std::option::Option;
use mongodb::bson::doc;
use mongodb::options::{UpdateOptions};
use std::string::String;

use serenity::model::application::command::Command;

struct Roles{
    admin: Option<RoleId>,
    moderator: Option<RoleId>,
    host: Option<RoleId>,
    verified: Option<RoleId>,
    registred: Option<RoleId>,
    everyone: Option<RoleId>,
}

impl Roles{
    pub fn new() -> Roles {
        Roles{
            admin: None,
            moderator: None,
            host: None,
            verified: None,
            registred: None,
            everyone: None
        }}
}

pub async fn setup_env_setup(ctx : &Context){
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(SETUP_ENV)
            .description("Permet de setup les salons d'un serveur.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await;
}


pub async fn setup_env(ctx: &Context, aci: &ApplicationCommandInteraction, mongo : &Client) {
    let guild = ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap();
    if !guild.features.contains(&"COMMUNITY".to_string()) {
        aci.create_interaction_response(&ctx, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed|{
                        embed.colour(RED_COLOR)
                            .title("Le serveur n'est pas \"communutaire\" !")
                            .description(
                                format!("Rends toi dans les paramètres > Communauté pour l'activer.\n\
                                    Paramètre tout par défaut, puis reviens ici faire la commande /setup."
                                )
                            )
                            .field("Plus d'infos",
                                   format!("Le salons **#{}** et le salon **#{}** pourrons selon ton souhait, servir de salon de substitution à ceux qui seront créés.\n\
                                   Les salons ne sont pas remplacés affin de ne pas casser une éventuelle mise en place précédente.\n\
                                   Les salons qui sont créés sont en revanche uniques par leur noms qui ne doit pas être modifié, et il n'est pas recommandé de les supprimer.",
                                           RULES_CHANNEL_NAME, MODERATION_CONVERSATION_CHANNEL_NAME),
                                   false
                            )
                    })
                })
        }).await.expect("Message non envoyé");
        return;
    }
    aci.create_interaction_response(&ctx, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|embed|{ 
                    embed.colour(BLUE_COLOR)
                        .title("Setup du serveur en cours...")
                        .description(
                            format!("Les catégories et les salons necessaires à la compétition sont en cours de création."
                            )
                        )
                }) 
            })
    }).await.unwrap();
    
    let mut founds = new_founds();
    
    //récupération de l'état des salons, catégories et rôles du serveur pour la (re)création du necessaire
    let channels = ctx.http.get_channels(aci.guild_id.unwrap().0).await.unwrap();
    founds = get_categories_existance(&channels, founds).await;
    founds = get_channels_existance(&channels, founds).await;
    founds = get_roles_existance(ctx.http.get_guild_roles(aci.guild_id.unwrap().0).await.unwrap(), founds).await;
    
    //Création des roles manquants
    let (admin, moderator, host, verified , inscrit, everyone) = create_roles(&ctx, &aci, &guild, &founds).await;
    founds.insert(ADMIN_ROLE_NAME, admin.0);
    founds.insert(MODERATION_ROLE_NAME, moderator.0);
    founds.insert(HOST_ROLE_NAME, host.0);
    founds.insert(VERIFIED_ROLE_NAME, verified.0);
    founds.insert(REGISTERED_ROLE_NAME, inscrit.0);
    founds.insert(EVERYONE_ROLE_NAME, everyone.0);
    
    //Création des catégories manquantes
    create_categories(&ctx, &aci, &guild, &mut founds).await;
    
    //TODO : save les ids dans la bdd
    
    let channels = ctx.http.get_channels(aci.guild_id.unwrap().0).await.unwrap();
    founds = get_categories_existance(&channels, founds).await;
    founds = get_channels_existance(&channels, founds).await;
    
    
    let filter = doc! {
        GUILD_ID: doc! {
            "$eq": &aci.guild_id.unwrap().0.to_string()
        }
    };
    
    let discord_info = doc! {
        "$set" : {
            GUILD_ID : &aci.guild_id.unwrap().0.to_string(),
            ADMIN_ROLE_ID     : &founds.get(ADMIN_ROLE_NAME).unwrap().to_string(),
            MODERATOR_ROLE_ID : &founds.get(MODERATION_ROLE_NAME).unwrap().to_string(),
            HOST_ROLE_ID      : &founds.get(HOST_ROLE_NAME).unwrap().to_string(),
            REGISTERED_ROLE_ID      : &founds.get(REGISTERED_ROLE_NAME).unwrap().to_string(),
            MODERATION_CATEGORY_ID  : &founds.get(MODERATION_CATEGORY_NAME).unwrap().to_string(),
            COMPETITION_CATEGORY_ID : &founds.get(COMPETITION_CATEGORY_NAME).unwrap().to_string(),
            REGISTRATION_CHANNEL_ID : &founds.get(REGISTRATION_CHANNEL_NAME).unwrap().to_string()
        }
    };
    let collection = mongo.database(RAYQUABOT_DB).collection::<String>(SERVER_COLLECTION);
    collection.update_one(filter, discord_info, UpdateOptions::builder().upsert(Some(true)).build()).await
        .expect(format!("Erreur a l'insertion des informations discord pour le serveur ! Envoie un mail à {} pour obtenir de l'aide.", CONTACT).as_str());
    
    
    
    aci.get_interaction_response(&ctx).await.unwrap().edit(&ctx, |message|{
        message.embed(|embed|{
            embed.colour(GREEN_COLOR)
                .title("Setup du serveur terminé !")
                .description(
                    format!("Les catégories et les salons necessaires à la compétition ont été créés avec succès.\n\
                        Pour ajouter une édition, rends toi dans le salon #{} puis tape la commande **/{}\n\
                        Pour éditer les versions autorisées ou non fait la commande /{}.\n\
                        Si tu as fait une erreur de saisie, tu peux également modifier ou supprimer les dates avec :\n\
                        /{} et /{}. Le nom ne peut être changé.", MODERATION_CONVERSATION_CHANNEL_NAME, NEW_EDITION, VERSION_SETUP, EDIT_EDITION, DELETE_EDITION
                    )
                )
        })
    }).await.unwrap();
}

async fn get_categories_existance<'a>(channels: &Vec<GuildChannel>,  mut map: HashMap<&'a str, u64>) -> HashMap<&'a str, u64> {
    for channel in channels {
        match channel.kind {
            Category => {
                match channel.name() {
                    GENERAL_CATEGORY_NAME => &map.insert(GENERAL_CATEGORY_NAME, channel.id.0),
                    MODERATION_CATEGORY_NAME => &map.insert(MODERATION_CATEGORY_NAME, channel.id.0),
                    COMPETITION_CATEGORY_NAME => &map.insert(COMPETITION_CATEGORY_NAME,  channel.id.0),
                    _ => &None
                };
            }
            _ => {}
        };
    }
    return map;
}

async fn get_channels_existance<'a>(channels: &Vec<GuildChannel>, mut map: HashMap<&'static str, u64>) -> HashMap<&'static str, u64> {
    for channel in channels {
        match channel.kind {
            Text => {
                if channel.parent_id.is_some() {
                    match channel.name() {
                        WAITING_VALIDATION_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(MODERATION_CATEGORY_NAME).unwrap() {
                                map.insert(WAITING_VALIDATION_CHANNEL_NAME, channel.id.0);
                        },
                        PROBLEMATIC_PLAYERS_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(MODERATION_CATEGORY_NAME).unwrap() {
                            map.insert(PROBLEMATIC_PLAYERS_CHANNEL_NAME, channel.id.0);
                        },
                        MODERATION_CONVERSATION_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(MODERATION_CATEGORY_NAME).unwrap() {
                            map.insert(MODERATION_CONVERSATION_CHANNEL_NAME, channel.id.0);
                        },
                        BAN_AND_EXCLUSION_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(MODERATION_CATEGORY_NAME).unwrap() {
                            map.insert(BAN_AND_EXCLUSION_CHANNEL_NAME, channel.id.0);
                        },
        
                        VALIDATED_CAPTURES_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(VALIDATED_CAPTURES_CHANNEL_NAME, channel.id.0);
                        },
                        RULES_AND_INFOS_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(RULES_AND_INFOS_CHANNEL_NAME, channel.id.0);
                        },
                        COMPETITION_ANNOUNCES_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(COMPETITION_ANNOUNCES_CHANNEL_NAME, channel.id.0);
                        },
                        TEAM_CREATION_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(TEAM_CREATION_CHANNEL_NAME, channel.id.0);
                        },
                        REGISTRATION_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(REGISTRATION_CHANNEL_NAME, channel.id.0);
                        },
                        RANKING_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(RANKING_CHANNEL_NAME, channel.id.0);
                        },
                        VERSIONS_AND_CHARMS_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap() {
                            map.insert(VERSIONS_AND_CHARMS_CHANNEL_NAME, channel.id.0);
                        },
                        BOT_RETURN_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(GENERAL_CATEGORY_NAME).unwrap() {
                            map.insert(BOT_RETURN_CHANNEL_NAME, channel.id.0);
                        },
                        WELCOME_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(GENERAL_CATEGORY_NAME).unwrap() {
                            map.insert(WELCOME_CHANNEL_NAME, channel.id.0);
                        },
                        GENERAL_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(GENERAL_CATEGORY_NAME).unwrap() {
                            map.insert(GENERAL_CHANNEL_NAME, channel.id.0);
                        },
                        RULES_CHANNEL_NAME => if channel.parent_id.unwrap().0 == *map.get(GENERAL_CATEGORY_NAME).unwrap() {
                            map.insert(RULES_CHANNEL_NAME, channel.id.0);
                        },
                        _ => ()
                    };
                }
            }
            Voice => {
                if (channel.name() == MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME && channel.parent_id.is_some()) && (channel.parent_id.unwrap().0 == *map.get(MODERATION_CATEGORY_NAME).unwrap()) {
                    let _ = &map.insert(MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME, channel.id.0);
                }
                if (channel.name() == GENERAL_VOCAL_CHANNEL_NAME && channel.parent_id.is_some()) && (channel.parent_id.unwrap().0 == *map.get(GENERAL_CATEGORY_NAME).unwrap()) {
                    let _ = &map.insert(GENERAL_VOCAL_CHANNEL_NAME, channel.id.0);
                }
            }
            //May break
            Forum=>
            if channel.parent_id.is_some() {
                match channel.name() {
                    FAQ_CHANNEL_NAME => {
                        if channel.parent_id.unwrap().0 == *map.get(COMPETITION_CATEGORY_NAME).unwrap(){
                            map.insert( FAQ_CHANNEL_NAME, channel.id.0);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        };
    }
    return map;
}

async fn get_roles_existance<'a>(roles: Vec<Role>, mut map: HashMap<&'static str, u64>) -> HashMap<&'static str, u64> {
    for role in roles{
        match role.name.as_str() {
            ADMIN_ROLE_NAME      => &map.insert(ADMIN_ROLE_NAME, role.id.0),
            MODERATION_ROLE_NAME => &map.insert(MODERATION_ROLE_NAME, role.id.0),
            HOST_ROLE_NAME       => &map.insert(HOST_ROLE_NAME, role.id.0),
            REGISTERED_ROLE_NAME => &map.insert(REGISTERED_ROLE_NAME, role.id.0),
            VERIFIED_ROLE_NAME => &map.insert(VERIFIED_ROLE_NAME, role.id.0),
            _ => &None
        };
    }
    
    return map;
}

fn new_founds() -> HashMap<&'static str, u64> {
    let mut founds = HashMap::new();
    founds.insert(MODERATION_CATEGORY_NAME, 0);
    founds.insert(WAITING_VALIDATION_CHANNEL_NAME, 0);
    founds.insert(PROBLEMATIC_PLAYERS_CHANNEL_NAME, 0);
    founds.insert(BAN_AND_EXCLUSION_CHANNEL_NAME, 0);
    founds.insert(MODERATION_CONVERSATION_CHANNEL_NAME, 0);
    founds.insert(MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME, 0);
    founds.insert(GENERAL_CATEGORY_NAME, 0);
    founds.insert(COMPETITION_CATEGORY_NAME, 0);
    founds.insert(RULES_AND_INFOS_CHANNEL_NAME, 0);
    founds.insert(COMPETITION_ANNOUNCES_CHANNEL_NAME, 0);
    founds.insert(RANKING_CHANNEL_NAME, 0);
    founds.insert(VALIDATED_CAPTURES_CHANNEL_NAME, 0);
    founds.insert(REGISTRATION_CHANNEL_NAME, 0);
    founds.insert(VERSIONS_AND_CHARMS_CHANNEL_NAME, 0);
    founds.insert(TEAM_CREATION_CHANNEL_NAME, 0);
    founds.insert(ADMIN_ROLE_NAME, 0);
    founds.insert(MODERATION_ROLE_NAME, 0);
    founds.insert(HOST_ROLE_NAME, 0);
    founds.insert(VERIFIED_ROLE_NAME, 0);
    founds.insert(REGISTERED_ROLE_NAME, 0);
    founds.insert(FAQ_CHANNEL_NAME, 0);
    founds.insert(BOT_RETURN_CHANNEL_NAME, 0);
    founds.insert(GENERAL_VOCAL_CHANNEL_NAME, 0);
    founds.insert(GENERAL_CHANNEL_NAME, 0);
    founds.insert(RULES_CHANNEL_NAME, 0);
    founds.insert(WELCOME_CHANNEL_NAME, 0);
    return founds;
}


async fn create_or_get_admin_role<'a>(ctx: &'a Context, aci: &'a ApplicationCommandInteraction, found: u64) -> RoleId {
    if found == 0{
        return ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap()
            .create_role(&ctx.http, |new_role| {
                new_role.name(ADMIN_ROLE_NAME)
                    .permissions(Permissions::ADMINISTRATOR)
                    .colour(0x24aff9)
            })
            .await.unwrap().id;
    }
    ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap().role_by_name(ADMIN_ROLE_NAME).unwrap().id
    
}
async fn create_or_get_moderator_role<'a>(ctx: &'a Context, aci: &'a ApplicationCommandInteraction, found: u64) -> RoleId {
    if found == 0{
        return ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap()
            .create_role(&ctx.http, |new_role|{
                new_role.name(MODERATION_ROLE_NAME)
                    .permissions(Permissions::MODERATE_MEMBERS|
                        Permissions::VIEW_CHANNEL |
                        Permissions::MENTION_EVERYONE |
                        Permissions::BAN_MEMBERS |
                        Permissions::CHANGE_NICKNAME |
                        Permissions::CONNECT |
                        Permissions::DEAFEN_MEMBERS |
                        Permissions::MANAGE_MESSAGES |
                        Permissions::KICK_MEMBERS |
                        Permissions::MUTE_MEMBERS |
                        Permissions::PRIORITY_SPEAKER |
                        Permissions::READ_MESSAGE_HISTORY |
                        Permissions::MOVE_MEMBERS |
                        Permissions::SEND_MESSAGES |
                        Permissions::SEND_MESSAGES_IN_THREADS |
                        Permissions::MANAGE_THREADS |
                        Permissions::USE_SLASH_COMMANDS
                    )
                    .colour(0xff191b)
            })
            .await.unwrap().id
    }
    
    ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap().role_by_name(MODERATION_ROLE_NAME).unwrap().id
}
async fn create_or_get_host_role<'a>(ctx: &'a Context, aci: &'a ApplicationCommandInteraction, found: u64) -> RoleId {
    if found == 0{
        return ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap()
            .create_role(&ctx.http, |new_role|{
                new_role.name(HOST_ROLE_NAME)
                    .permissions(Permissions::MODERATE_MEMBERS |
                        Permissions::MENTION_EVERYONE |
                        Permissions::CHANGE_NICKNAME |
                        Permissions::PRIORITY_SPEAKER |
                        Permissions::READ_MESSAGE_HISTORY |
                        Permissions::MOVE_MEMBERS |
                        Permissions::SEND_MESSAGES |
                        Permissions::SEND_MESSAGES_IN_THREADS |
                        Permissions::USE_SLASH_COMMANDS
                    )
                    .colour(0xff5d47)
            })
            .await.unwrap().id;
    }
    
    ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap().role_by_name(HOST_ROLE_NAME).unwrap().id
}
async fn create_or_get_verified_role<'a>(ctx: &'a Context, aci: &'a ApplicationCommandInteraction, found: u64) -> RoleId {
    if found == 0{
        return ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap()
            .create_role(&ctx.http, |new_role|{
                new_role.name(VERIFIED_ROLE_NAME)
                    .colour(0x0a8800)
                    .permissions(Permissions::empty())
            })
            .await.unwrap().id;
    }
    ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap().role_by_name(VERIFIED_ROLE_NAME).unwrap().id
}
async fn create_or_get_inscrit_role<'a>(ctx: &'a Context, aci: &'a ApplicationCommandInteraction, found: u64) -> RoleId {
    if found == 0{
        return ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap()
            .create_role(&ctx.http, |new_role|{
                new_role.name(REGISTERED_ROLE_NAME)
                    .colour(0x4f8342)
                    .permissions(Permissions::empty())
            })
            .await.unwrap().id;
    }
    ctx.http.get_guild(aci.guild_id.unwrap().0).await.unwrap().role_by_name(REGISTERED_ROLE_NAME).unwrap().id
}



async fn create_validation_en_attente_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel {
    let permissions = vec![
        PermissionOverwrite {
            allow:
                Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }        
    ];
    create_text_channel(&ctx, &aci, permissions, WAITING_VALIDATION_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_joueurs_a_problemes_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.moderator.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, PROBLEMATIC_PLAYERS_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_ban_et_exclusions_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.moderator.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, BAN_AND_EXCLUSION_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_discussions_moderation_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.moderator.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, MODERATION_CONVERSATION_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_discussion_vocal_moderation_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow:
                Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES |
                Permissions::CONNECT |
                Permissions::MOVE_MEMBERS |
                Permissions::MUTE_MEMBERS |
                Permissions::KICK_MEMBERS |
                Permissions::DEAFEN_MEMBERS |
                Permissions::SPEAK |
                Permissions::STREAM,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow:
                Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE |
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES |
                Permissions::CONNECT |
                Permissions::MOVE_MEMBERS |
                Permissions::MUTE_MEMBERS |
                Permissions::KICK_MEMBERS |
                Permissions::DEAFEN_MEMBERS |
                Permissions::SPEAK |
                Permissions::STREAM,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(roles.moderator.unwrap())
        },
        PermissionOverwrite {
            allow:
                Permissions::ADD_REACTIONS |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS |
                Permissions::SEND_MESSAGES |
                Permissions::SPEAK |
                Permissions::STREAM,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_voice_channel(&ctx, &aci, permissions, MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME, *parent_category, existance).await
}

async fn create_moderation_category(ctx: &Context, aci: &ApplicationCommandInteraction, roles: &Roles) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow:
            Permissions::VIEW_CHANNEL |
                Permissions::MENTION_EVERYONE,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(roles.moderator.unwrap())
        },
        PermissionOverwrite {
            allow:Permissions::empty(),
            deny:Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        },
            PermissionOverwrite {
            allow:Permissions::empty(),
            deny:Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow:Permissions::empty(),
            deny:Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        }
    ];
    create_category(&ctx, &aci, permissions, MODERATION_CATEGORY_NAME).await
}
async fn create_moderation_setup(ctx: &Context, aci: &ApplicationCommandInteraction, guild: &PartialGuild, map: &mut HashMap<&str, u64>) {
    let mut roles = Roles::new();
    roles.everyone = Option::from(RoleId::from(*map.get(EVERYONE_ROLE_NAME).unwrap()));
    roles.moderator = Option::from(RoleId::from(*map.get(MODERATION_ROLE_NAME).unwrap()));
    roles.host = Option::from(RoleId::from(*map.get(HOST_ROLE_NAME).unwrap()));
    roles.verified = Option::from(RoleId::from(*map.get(VERIFIED_ROLE_NAME).unwrap()));
    roles.registred = Option::from(RoleId::from(*map.get(REGISTERED_ROLE_NAME).unwrap()));
    roles.admin = Option::from(RoleId::from(*map.get(ADMIN_ROLE_NAME).unwrap()));
    
    let mut moderation_category_id = *map.get(MODERATION_CATEGORY_NAME).unwrap();
    if moderation_category_id == 0 {
        moderation_category_id = create_moderation_category(&ctx, &aci, &roles).await.id.0;
    }
    
    let validation_en_attente_existance = *map.get(WAITING_VALIDATION_CHANNEL_NAME).unwrap();
    let validation_en_attente_channel = create_validation_en_attente_channel(&ctx, &aci, &moderation_category_id, &roles, validation_en_attente_existance);
    
    let joueur_a_probleme_existance = *map.get(PROBLEMATIC_PLAYERS_CHANNEL_NAME).unwrap();
    let joueur_a_probleme_channel = create_joueurs_a_problemes_channel(&ctx, &aci, &moderation_category_id, &roles, joueur_a_probleme_existance);
    
    let ban_et_exclusion_existance = *map.get(BAN_AND_EXCLUSION_CHANNEL_NAME).unwrap();
    let ban_et_exclusions_channel = create_ban_et_exclusions_channel(&ctx, &aci, &moderation_category_id, &roles, ban_et_exclusion_existance);
    
    let discussion_moderation_text_existance = *map.get(MODERATION_CONVERSATION_CHANNEL_NAME).unwrap();
    let discussion_moderation_text_channel = create_discussions_moderation_channel(&ctx, &aci, &moderation_category_id, &roles,discussion_moderation_text_existance);
    
    let discussion_moderation_voice_existance = *map.get(MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME).unwrap();
    let discussion_moderation_voice_channel = create_discussion_vocal_moderation_channel(&ctx, &aci, &moderation_category_id, &roles, discussion_moderation_voice_existance);
    
    
    let channels = join!(validation_en_attente_channel, joueur_a_probleme_channel, ban_et_exclusions_channel, discussion_moderation_text_channel, discussion_moderation_voice_channel);
    
    let order = vec![
        (channels.0.id, 0),
        (channels.1.id, 1),
        (channels.2.id, 2),
        (channels.3.id, 3),
        (channels.4.id, 4),
    ];
    guild.reorder_channels(&ctx.http, order).await.unwrap();
}


async fn create_acceuil_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, WELCOME_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_rules_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, RULES_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_general_vocal_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::CONNECT |
            Permissions::STREAM |
            Permissions::SPEAK |
            Permissions::USE_EMBEDDED_ACTIVITIES |
            Permissions::USE_VAD,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_voice_channel(&ctx, &aci, permissions, GENERAL_VOCAL_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_general_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, GENERAL_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_retour_bot_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, BOT_RETURN_CHANNEL_NAME, *parent_category, existance).await
}

async fn create_general_category(ctx: &Context, aci: &ApplicationCommandInteraction, roles: &Roles) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::all(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_category(&ctx, &aci, permissions, GENERAL_CATEGORY_NAME).await
}
async fn create_general_setup(ctx: &Context, aci: &ApplicationCommandInteraction, guild: &PartialGuild, map: &mut HashMap<&str, u64>) {
    let mut roles = Roles::new();
    roles.everyone = Option::from(RoleId::from(*map.get(EVERYONE_ROLE_NAME).unwrap()));
    roles.moderator = Option::from(RoleId::from(*map.get(MODERATION_ROLE_NAME).unwrap()));
    roles.host = Option::from(RoleId::from(*map.get(HOST_ROLE_NAME).unwrap()));
    roles.verified = Option::from(RoleId::from(*map.get(VERIFIED_ROLE_NAME).unwrap()));
    roles.registred = Option::from(RoleId::from(*map.get(REGISTERED_ROLE_NAME).unwrap()));
    roles.admin = Option::from(RoleId::from(*map.get(ADMIN_ROLE_NAME).unwrap()));
    
    let mut general_category_id = *map.get(GENERAL_CATEGORY_NAME).unwrap();
    if general_category_id == 0 {
        general_category_id = create_general_category(&ctx, &aci, &roles).await.id.0;
    }
    
    let retour_bot_existance = *map.get(BOT_RETURN_CHANNEL_NAME).unwrap();
    let retour_bot = create_retour_bot_channel(&ctx, &aci, &general_category_id, &roles, retour_bot_existance);
    
    let general_voice_existance = *map.get(GENERAL_VOCAL_CHANNEL_NAME).unwrap();
    let general_voice_channel = create_general_vocal_channel(&ctx, &aci, &general_category_id, &roles, general_voice_existance);
    
    let general_text_existance = *map.get(GENERAL_CHANNEL_NAME).unwrap();
    let general_text_channel = create_general_channel(&ctx, &aci, &general_category_id, &roles, general_text_existance);
     
    let reglement_existance = *map.get(RULES_CHANNEL_NAME).unwrap();
    let reglement_channel = create_rules_channel(&ctx, &aci, &general_category_id, &roles, reglement_existance);
    
    let acceuil_existance = *map.get(WELCOME_CHANNEL_NAME).unwrap();
    let acceuil_channel = create_acceuil_channel(&ctx, &aci, &general_category_id, &roles, acceuil_existance);
    
    
    let (retour, general_text, _general_voice, reglement, acceuil) = join!(retour_bot, general_text_channel, general_voice_channel, reglement_channel, acceuil_channel);
    let order = vec![
        (acceuil.id, 0),
        (reglement.id, 1),
        (retour.id, 3),
        (general_text.id, 4)
    ];
    guild.reorder_channels(&ctx.http, order).await.unwrap();
}

/*
L'Arène                        //catégorie visible par tous
  règlement et infos           //tout le monde
  annonces                     //tout le monde
  classement                   //tout le monde
  captures validées            //tout le monde
  faq                          //tout le monde
  1 inscriptions               //Tous ceux qui n'ont pas le role inscrit
  2 versions et charme chroma  //tout ceux qui ont le rôle Inscrit et hosts
  3 constitution des équipes   //tous ceux qui ont le rôle Vérifié et hosts

        gimiks                       //tout le monde, change chaque semaine
 */
async fn create_reglement_et_infos_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, RULES_AND_INFOS_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_annonces_competition_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
            Permissions::SEND_MESSAGES |
            Permissions::MENTION_EVERYONE,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, COMPETITION_ANNOUNCES_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_classement_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, RANKING_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_captures_validees_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, VALIDATED_CAPTURES_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_faq_forum(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    if existance == 0{
        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL |
                    Permissions::SEND_MESSAGES |
                    Permissions::SEND_MESSAGES_IN_THREADS |
                    Permissions::CREATE_PUBLIC_THREADS,
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
            }
        ];
        
        let id = aci.guild_id.unwrap().0;
        return ctx.http.get_guild(id).await.unwrap()
            .create_channel(&ctx.http, |channel| {
                channel.name(FAQ_CHANNEL_NAME)
                    .kind(Forum)
                    .permissions(permissions)
                    .category(*parent_category)
            })
            .await
            .expect(format!("Le salon textuel {} n'as pas pu être créer", FAQ_CHANNEL_NAME).as_ref());
    }
    return ctx.http.get_channel(existance)
        .await
        .expect(format!("Impossible de récupérer le channel {}", FAQ_CHANNEL_NAME).as_ref())
        .guild().expect(format!("Impossible de récupérer le GuildChannel de {}", FAQ_CHANNEL_NAME).as_ref())
}
async fn create_registration_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::MENTION_EVERYONE,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, REGISTRATION_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_versions_et_charmes_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::MENTION_EVERYONE,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, VERSIONS_AND_CHARMS_CHANNEL_NAME, *parent_category, existance).await
}
async fn create_constitution_equipes_channel(ctx: &Context, aci: &ApplicationCommandInteraction, parent_category: &u64, roles: &Roles, existance: u64) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::USE_SLASH_COMMANDS,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.verified.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL |
                Permissions::SEND_MESSAGES |
                Permissions::READ_MESSAGE_HISTORY |
                Permissions::MENTION_EVERYONE,
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.host.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.registred.unwrap())
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_text_channel(&ctx, &aci, permissions, TEAM_CREATION_CHANNEL_NAME, *parent_category, existance).await
}

async fn create_competition_category(ctx: &Context, aci: &ApplicationCommandInteraction, roles: &Roles) -> GuildChannel{
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::all(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(roles.everyone.unwrap())
        }
    ];
    create_category(&ctx, &aci, permissions, COMPETITION_CATEGORY_NAME).await
}
async fn create_competition_setup(ctx: &Context, aci: &ApplicationCommandInteraction, guild: &PartialGuild, map: &mut HashMap<&str, u64>) {
    let mut roles = Roles::new();
    roles.everyone = Option::from(RoleId::from(*map.get(EVERYONE_ROLE_NAME).unwrap()));
    roles.moderator = Option::from(RoleId::from(*map.get(MODERATION_ROLE_NAME).unwrap()));
    roles.host = Option::from(RoleId::from(*map.get(HOST_ROLE_NAME).unwrap()));
    roles.verified = Option::from(RoleId::from(*map.get(VERIFIED_ROLE_NAME).unwrap()));
    roles.registred = Option::from(RoleId::from(*map.get(REGISTERED_ROLE_NAME).unwrap()));
    roles.admin = Option::from(RoleId::from(*map.get(ADMIN_ROLE_NAME).unwrap()));
    
    let mut competition_category_id = *map.get(COMPETITION_CATEGORY_NAME).unwrap();
    if competition_category_id == 0 {
        competition_category_id = create_competition_category(&ctx, &aci, &roles).await.id.0;
    }
    
    let reglement_et_info_existance = *map.get(RULES_AND_INFOS_CHANNEL_NAME).unwrap();
    let reglement_et_info_channel = create_reglement_et_infos_channel(&ctx, &aci, &competition_category_id, &roles, reglement_et_info_existance);
    
    let annonces_existance = *map.get(COMPETITION_ANNOUNCES_CHANNEL_NAME).unwrap();
    let annonces_channel = create_annonces_competition_channel(&ctx, &aci, &competition_category_id, &roles, annonces_existance);
    
    let classement_existance = *map.get(RANKING_CHANNEL_NAME).unwrap();
    let classement_channel = create_classement_channel(&ctx, &aci, &competition_category_id, &roles, classement_existance);
    
    let captures_existance = *map.get(VALIDATED_CAPTURES_CHANNEL_NAME).unwrap();
    let captures_channel = create_captures_validees_channel(&ctx, &aci, &competition_category_id, &roles, captures_existance);
    
    let registrations_existance = *map.get(REGISTRATION_CHANNEL_NAME).unwrap();
    let registration_channel = create_registration_channel(&ctx, &aci, &competition_category_id, &roles, registrations_existance);
    
    let versions_et_charmes_existance = *map.get(VERSIONS_AND_CHARMS_CHANNEL_NAME).unwrap();
    let versions_et_charmes_channel = create_versions_et_charmes_channel(&ctx, &aci, &competition_category_id, &roles, versions_et_charmes_existance);
    
    let equipes_existance = *map.get(TEAM_CREATION_CHANNEL_NAME).unwrap();
    let equipes_channel = create_constitution_equipes_channel(&ctx, &aci, &competition_category_id, &roles, equipes_existance);
    
    let faq_existance = *map.get(FAQ_CHANNEL_NAME).unwrap();
    let faq_channel = create_faq_forum(&ctx, &aci, &competition_category_id, &roles, faq_existance);
    
    let (registrations, versions, equipes) = join!(registration_channel, versions_et_charmes_channel, equipes_channel);
    let (reglement, annonces, classement, captures, faq) = join!(reglement_et_info_channel, annonces_channel, classement_channel, captures_channel, faq_channel);
    
    let order = vec![
        (reglement.id, 0),
        (faq.id, 1),
        (annonces.id, 2),
        (classement.id, 3),
        (captures.id, 4),
        (registrations.id, 5),
        (versions.id, 6),
        (equipes.id, 7),
    ];
    guild.reorder_channels(&ctx.http, order).await.unwrap();
}


async fn create_categories(ctx: &Context, aci: &ApplicationCommandInteraction, guild: &PartialGuild, map: &mut HashMap<&str, u64>){
    create_moderation_setup(&ctx, &aci, &guild, map).await;
    create_general_setup(&ctx, &aci, &guild, map).await;
    create_competition_setup(&ctx, &aci, &guild, map).await;
}


async fn create_roles(ctx: &Context, aci: &ApplicationCommandInteraction, guild: &PartialGuild, map: &HashMap<&str, u64>) -> (RoleId, RoleId, RoleId, RoleId, RoleId, RoleId) {
    let admin = create_or_get_admin_role(&ctx, &aci, *map.get(ADMIN_ROLE_NAME).unwrap());
    let moderator = create_or_get_moderator_role(&ctx, &aci, *map.get(MODERATION_ROLE_NAME).unwrap());
    let host = create_or_get_host_role(&ctx, &aci, *map.get(HOST_ROLE_NAME).unwrap());
    let verified = create_or_get_verified_role(&ctx, &aci, *map.get(VERIFIED_ROLE_NAME).unwrap());
    let inscrit = create_or_get_inscrit_role(&ctx, &aci, *map.get(REGISTERED_ROLE_NAME).unwrap());
    let everyone = (&ctx).http.get_guild(aci.guild_id.unwrap().0);
    
    let (admin, moderator, host, verified, inscrit, everyone) = join!(admin, moderator, host, verified, inscrit, everyone);
    let everyone = everyone.unwrap().role_by_name("@everyone").unwrap().id;
    
    let _ = guild.edit_role_position(&ctx, &everyone, 0).await;
    let _ = guild.edit_role_position(&ctx, &inscrit, 1).await;
    let _ = guild.edit_role_position(&ctx, &verified, 2).await;
    let _ = guild.edit_role_position(&ctx, &host, 3).await;
    let _ = guild.edit_role_position(&ctx, &moderator, 4).await;
    let _ = guild.edit_role_position(&ctx, &admin, 5).await;
    
    
    return (admin, moderator, host, verified, inscrit, everyone);
}
async fn create_category(ctx: &Context, aci: &ApplicationCommandInteraction, permissions: Vec<PermissionOverwrite>, channel_name: &str) -> GuildChannel {
    let id = aci.guild_id.unwrap().0;
    return ctx.http.get_guild(id).await.unwrap()
        .create_channel(&ctx.http, |channel| {
            channel.name(channel_name)
                .kind(Category)
                .permissions(permissions)
        })
        .await
        .expect(format!("La catégorie {} n'as pas pu être créer", channel_name).as_ref());
}
async fn create_text_channel(ctx: &Context, aci: &ApplicationCommandInteraction, permissions: Vec<PermissionOverwrite>, channel_name: &str, parent_category: u64, existance: u64) -> GuildChannel {
    if existance != 0 {
        return ctx.http.get_channel(existance)
            .await
            .expect(format!("Impossible de récupérer le channel {}", channel_name).as_ref())
            .guild().expect(format!("Impossible de récupérer le GuildChannel de {}", channel_name).as_ref())
    }
    
    
    
    let id = aci.guild_id.unwrap().0;
    return ctx.http.get_guild(id).await.unwrap()
        .create_channel(&ctx.http, |channel| {
            channel.name(channel_name)
                .kind(Text)
                .permissions(permissions)
                .category(parent_category)
        })
        .await
        .expect(format!("Le salon textuel {} n'as pas pu être créer", channel_name).as_ref());
}
async fn create_voice_channel(ctx: &Context, aci: &ApplicationCommandInteraction, permissions: Vec<PermissionOverwrite>, channel_name: &str, parent_category: u64, existance: u64) -> GuildChannel {
    if existance != 0 {
        return ctx.http.get_channel(existance)
            .await
            .expect(format!("Impossible de récupérer le salon {}", channel_name).as_ref())
            .guild().expect(format!("Impossible de récupérer le GuildChannel de {}", channel_name).as_ref())
    }
    
    let id = aci.guild_id.unwrap().0;
    return ctx.http.get_guild(id).await.unwrap()
        .create_channel(&ctx.http, |channel| {
            channel.name(channel_name)
                .kind(Voice)
                .permissions(permissions)
                .category(parent_category)
        })
        .await
        .expect(format!("Le salon vocal {} n'as pas pu être créer", channel_name).as_ref());
}

/*
//catégories et channels
Arbitrage                      //admin, host, et modérateurs
  validations en attente       //seuls les hosts peuvent valider les captures
  joueurs a problemes
  bans et exclusions
  discussions
  discussions (vocal)
  
Gradins                        //tout le monde
  acceuil
  règlement
  retour bot                   //tout le monde
  général text
  général vocal
  
L'Arène                        //catégorie visible par tous
  règlement et infos           //tout le monde
  annonces                     //tout le monde
  classement                   //tout le monde
  captures validées            //tout le monde
  faq                          //tout le monde
  1 inscriptions               //Tous ceux qui n'ont pas le role inscrit
  2 versions et charme chroma  //tout ceux qui ont le rôle Inscrit et hosts
  3 constitution des équipes   //tous ceux qui ont le rôle Vérifié et hosts
  gimiks                       //tout le monde, change chaque semaine
  
[Nom d'équipe]                 //ceux qui ont le role de l'équipe
  proposition capture
  ecrit
  vocal (vocal)

//roles
Admin //peux voir tous les salons
Host  //peut voir les salons de modération
Modérateur //peut voir tous les salons
Vérifié
Inscrit
[noms d'équipes]
@everyone
*/
