use mongodb::Client;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::builder::CreateEmbed;
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command, CommandOptionType};
use mongodb::{Client as MongoClient};
use serenity::model::application::component::InputTextStyle;
use crate::doc;

pub async fn new_edition_setup(ctx: &Context) {
    let _ = Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("new_edition")
            .description("Create a new edition")
            .create_option(|option| {
                option
                    .name("name")
                    .description("Name of the edition")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("number")
                    .description("Number of the edition")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    })
        .await;
}

pub async fn new_edition_insertion(client : &Client, command : &ApplicationCommandInteraction) {
    let name = command.data.options.get(0).unwrap().value.as_ref().unwrap().to_string();
    let numero = command.data.options.get(1).unwrap().value.as_ref().unwrap().to_string();
    let edition = name.to_string() + " " + &*numero.to_string();
    let organisateur = command.user.id.0.to_string();

    let collection =  client.database("RayquaBot").collection("editions");
    let doc = doc! {
            "organisateur" : organisateur,
            "edition": edition,
            "date_debut_inscription": "",
            "date_fin_inscription": "",
            "date_debut_competition": "",
            "date_fin_competition": ""
        };
    collection.insert_one(doc, None).await.expect("Failed to insert document");
}

pub async unsafe fn new_edition_reactor(client : &MongoClient, command : &ApplicationCommandInteraction, context : &Context) {
    let ctx = context;
    let com = &command.clone();
    new_edition_insertion(client, com).await;
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message| message.components(
                |components| components.create_action_row(|action_row| {
                    action_row.create_input_text(|input_text| {
                        input_text
                            .custom_id("date_debut_inscriptions")
                            .placeholder("Date de début des inscriptions")
                            .min_length(10)
                            .max_length(10)
                            .required(true)
                            .label("Date de début des inscriptions")
                            .style(InputTextStyle::Short)
                    })
                })
            ).title("date_debut_inscriptions")
                .custom_id("modal_app_cmd")
            )
    })
        .await
        .expect("Failed to send interaction response");
}

pub async fn prompt_date_modal(mci : ModalSubmitInteraction, ctx : serenity::client::Context, date_type : String) {
    dbg!(&mci);
    let short_text = match mci
        .data
        .components
        .get(0)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    mci.create_interaction_response(ctx, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.add_embed(
                    CreateEmbed::default()
                        .title("Modal Response")
                        .description(format!("You said: {}", short_text.value)).to_owned(),
                )
            })
    })
        .await
        .unwrap();
}

