use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::{CreateEmbed};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient};
use mongodb::bson::Document;
use chrono;
use serenity::futures::StreamExt;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::Permissions;
use crate::doc;
use super::common_functions::*;
use super::constants::*;

pub async fn delete_edition_setup(ctx: &Context) {
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name(DELETE_EDITION)
            .description("Delete an existing edition on this server created by you")
            .description_localized("fr", "Supprime une édition que tu as créé sur ce serveur.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await;
}

pub async fn delete_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context) {
    let ctx = context;
    let com = &command.clone();

    let user_id = com.user.id.to_string();

    let editions = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).aggregate(
        [doc! {
            "$match": doc! {
                ORGANISATOR: &user_id.as_str(),
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
        let msg = format!("Tu n'as aucune édition en cours ou future qui puisse être supprimée,\
        \net tu ne peux pas supprimer une édition qui a déjà eu lieu.\
        \n\nPour toute demande de suppression d'édition passée, contacte le développeur à l'adresse mail **{}**", CONTACT);
        send_error_from_command(&com, &ctx, &msg).await;
        return;
    }

    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|
                message.components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_select_menu(|select_menu| {
                            select_menu.custom_id(DELETE_EDITION_MODAL)
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

pub async fn delete_edition_modal(client : &MongoClient, mci : MessageComponentInteraction, ctx : serenity::client::Context){
    ctx.http.delete_message(mci.channel_id.0, mci.message.id.0).await.unwrap();
    let result = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).delete_one(
        doc! {
            EDITION_NAME: mci.data.values.get(0).unwrap().as_str()
        },
        None
    ).await.unwrap();

    if result.deleted_count == 0 {
        let msg = format!("Une erreur est survenue lors de la suppression de l'édition.\
        \ncontacte le développeur à l'adresse mail **{}**", CONTACT);
        send_error_from_component(&mci, &ctx, &msg).await;
    }
    else {
        mci.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.add_embed(
                        CreateEmbed::default()
                            .title("delete_edition :")
                            .description("L'édition à été supprimée avec succès !").to_owned()
                            .color(GREEN_COLOR)
                            .to_owned()
                    )
                })
        })
            .await
            .unwrap();
    }
}