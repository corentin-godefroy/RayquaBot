use crate::commands::common_functions::{get_versions_list_tuple, send_error_from_command};
use crate::commands::constants::*;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::id::RoleId;
use std::str::FromStr;
use tokio::join;

pub async fn registration_setup(ctx: &Context) {
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(REGISTRATION)
            .description("Sers à s'inscrire à une édition.")
    })
    .await;
}

fn interdiction_to_emote<'a>(edition: &'a Document, field: &'a str) -> &'a str {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction {
        0 => "⛔",
        _ => "❌",
    }
}

fn interdiction_to_value<'a>(edition: &'a Document, field: &'a str) -> i32 {
    let interdiction = edition.get(field).unwrap().as_i32().unwrap();
    match interdiction {
        0 => VERSION_INTERDITE_VALUE,
        _ => NON_POSSEDE_VALUE,
    }
}

pub async fn get_registration_reactor(
    mongo: &Client,
    aci: &ApplicationCommandInteraction,
    ctx: &Context,
) {
    let filter = doc! {
        GUILD_ID: doc!{
            "$eq": &aci.guild_id.unwrap().0.to_string()
        }
    };

    let setup = mongo
        .database(RAYQUABOT_DB)
        .collection::<String>(SERVER_COLLECTION)
        .find(filter, None)
        .await
        .unwrap();
    if setup.current().is_empty() {
        send_error_from_command(&aci, &ctx, "Le serveur n'est pas setup. Demande à un membre ayant les permissions admin de faire la commande **__/setup__**").await;
        return;
    }

    let registration_channel_id = setup
        .current()
        .get(REGISTRATION_CHANNEL_ID)
        .unwrap()
        .unwrap()
        .as_str()
        .unwrap();
    if aci.channel_id.0.to_string() != registration_channel_id {
        let message = aci.member.as_ref().unwrap().user.dm(&ctx.http, |message|{
            message.add_embed(|embed|{
                embed.colour(RED_COLOR)
                    .title("Inscription")
                    .description(format!("Pour t'inscrire, il fait faire /{} dans le salon {}. Si tu ne vois pas ce salon et que tu n'est pas déjà inscrit, demande à un admin.", REGISTRATION, REGISTRATION_CHANNEL_NAME).as_str())
            })
        });

        let remove = aci.delete_original_interaction_response(&ctx.http);
        let _res = join!(message, remove);
        return;
    }

    let registred_role_id = RoleId::from_str(
        setup
            .current()
            .get(REGISTERED_ROLE_ID)
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();
    if aci
        .member
        .as_ref()
        .unwrap()
        .roles
        .contains(&registred_role_id)
    {
        send_error_from_command(&aci, &ctx, "Tu est déjà isncrit à la compétition !").await;
        return;
    }

    let timestamp: i64 = (((aci.id.0 >> 22) + 1420070400000) / 1000 as u64) as i64;

    let filter = doc! {
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

    let edition = mongo
        .database(RAYQUABOT_DB)
        .collection::<Document>(EDITIONS_COLLECTION)
        .find_one(filter, None)
        .await
        .unwrap()
        .unwrap();

    let start_registration = edition
        .get(INSCRIPTION_START_DATE)
        .unwrap()
        .as_i64()
        .unwrap();
    let end_registration = edition.get(INSCRIPTION_END_DATE).unwrap().as_i64().unwrap();

    if !(timestamp > start_registration && timestamp < end_registration) {
        aci.create_interaction_response(&ctx.http, |intearction| {
            intearction
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed
                            .colour(RED_COLOR)
                            .title("Inscriptions closes")
                            .description(
                                "Aucune édition n'est actuellement en phase d'inscription.",
                            )
                    })
                })
        })
        .await
        .unwrap();
        return;
    }

    let edition_name = edition.get(EDITION_NAME).unwrap().as_str().unwrap();

    aci.user.dm(&ctx.http, |dm|{
        dm.content(format!("__**`Pour continuer ta vérification, ça se passe ici. Voici un petit guide pour t'aider dans cette tache.`**__
- La commande **/{}** sers à indiquer **__TOUS__** les noms de dresseur utilisé dans les versions que tu possède. Inutile d'indiquer la version.
- La commande **/{}** vas te permettre d'indiquer par génération les versions que tu possède ainsi que si tu a AU MOINS UN charme chroma pour une version concerné.
    Autrement dit, si tu as plusieurs fois la même version et que tu possède le charme sur au moins l'une d'elle, tu dois indiquer que tu possède le charme.
    Par ailleurs il est recommandé d'indiquer TOUTES les versions possédés afin de simplifier les futures inscriptions.
- La commande **{}** sers quand à elle à fournir les preuves de possessions des versions pokémon.
    Les formats acceptés sont : liens youtube, liens google drive (drive.google.com) et les photos dirrectement via discord.

=> Si tu as besoin d'aide ou que tu as un doute, n'hésite pas à demander aux Host/Admin sur le serveur concerné.
    ❌ signifie que tu ne possède pas la version. C'est également la valeur par défaut des champs.
    ✅ signifie que tu possède la version SANS charme chroma.
    ✅💫 signifie que tu possède la version AVEC au moins 1 charme chroma.

_Voici le récap des infos :_", ADD_NAMES, VERSION_PLAYER_SETUP, ADD_PROOF))
    }).await.unwrap();

    let versions = get_versions_list_tuple();

    let message = aci
        .user
        .dm(&ctx.http, |dm| {
            dm.embed(|embed| {
                embed
                    .colour(LIGHT_BLUE_COLOR)
                    .title(format!(
                        "Récap de l'inscription pour l'édition {}",
                        edition_name
                    ))
                    .description("Toutes les infos que tu auras indiqués se trouvent ici.");
                if versions.len() % 2 == 1 {
                    for i in (0..versions.len() - 1).step_by(2) {
                        embed.field(
                            format!("{} : \n{}\n--------------------------", versions[i].1, "❌"),
                            format!(
                                "**{} : \n{}\n--------------------------**",
                                versions[i + 1].1,
                                "❌"
                            ),
                            true,
                        );
                    }
                    embed.field(
                        format!(
                            "{} : \n{}\n--------------------------",
                            versions[versions.len() - 1].1,
                            "❌"
                        ),
                        "".to_string(),
                        true,
                    );
                } else {
                    for i in (0..versions.len() - 1).step_by(2) {
                        embed.field(
                            format!("{} : \n{}\n--------------------------", versions[i].1, "❌"),
                            format!(
                                "**{} : \n{}\n--------------------------**",
                                versions[i + 1].1,
                                "❌"
                            ),
                            true,
                        );
                    }
                }
                embed.footer(|pied| pied.text("noms de dresseur : "))
            })
            .components(|component| {
                component.create_action_row(|action_row| {
                    action_row.create_button(|button| {
                        button
                            .custom_id(VALIDATE)
                            .style(ButtonStyle::Danger)
                            .label("Valider définitivement")
                    })
                })
            })
        })
        .await
        .unwrap();

    let mut player = doc! {
        PLAYER_ID: aci.user.id.0.to_string(),
        EDITION_NAME: edition.get(EDITION_NAME).unwrap().as_str().unwrap(),
        GUILD_ID: aci.guild_id.unwrap().0.to_string(),
        TEAM: None::<String>, //id du role de la team
        VERIFIED: false, //booléen pour déterminer si le joueur a été validé ou pas encore.
        MESSAGE_ID: message.id.0.to_string(),
        TRAINER_NAMES: "",
        MORE_INFO: "",
    };

    for version in versions {
        player.insert(version.0, NON_POSSEDE_VALUE);
    }

    mongo
        .database(RAYQUABOT_DB)
        .collection(PLAYER_COLLECTION)
        .insert_one(player, None)
        .await
        .expect("L'insertion d'un nouveau joueur à échoué.");

    let mut member = aci.member.clone().unwrap();

    member.add_role(&ctx.http, registred_role_id).await.unwrap();

    let name = edition.get(EDITION_NAME).unwrap().as_str().unwrap();

    aci.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|embed| {
                    embed
                        .colour(GREEN_COLOR)
                        .title("Inscription validée")
                        .description(format!(
                            "{} s'est inscrit avec succès à l'édition **{}**",
                            aci.user.name, name
                        ))
                })
            })
    })
    .await
    .unwrap();
}
