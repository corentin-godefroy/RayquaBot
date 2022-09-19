use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;

pub async fn ping_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name("ping").description("Reply with Pong")
    })
        .await.expect("Creation of ping command failed : ");
}

pub async fn ping_reactor(command: &ApplicationCommandInteraction, ctx: &Context) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Pong!"))
        })
        .await
        .expect("Failed to send interaction response");
}