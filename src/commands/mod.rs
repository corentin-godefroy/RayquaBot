pub mod commands {
    use std::borrow::Borrow;
    use lazy_static::lazy_static;
    use serenity::client::Context;
    use serenity::model::application::command::{Command, CommandOptionType};
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::model::application::interaction::InteractionResponseType;
    use mongodb::{Client as MongoClient, Client};
    use mongodb::options::ClientOptions;
    use crate::{new_edition_insertion};




    pub async fn ping_setup(ctx: &Context) {
        Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Reply with Pong")
        })
            .await.expect("Creation of ping command failed : ");
    }

    pub async fn ping_reactor(command: ApplicationCommandInteraction, ctx: Context) {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Pong!"))
            })
            .await
            .expect("Failed to send interaction response");
    }

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

    pub async unsafe fn new_edition_reactor(client : &Client, command : ApplicationCommandInteraction, context : Context) {
        let mut cmd = command.clone();
        let mut ctx = &context.clone();
        new_edition_insertion(client, cmd, ctx.clone()).await;
        command.create_interaction_response(ctx.clone().http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Edition ajoutée")
                })
        }).await.unwrap();
    }
}