pub mod command_reactor {

    use serenity::client::Context;
    use mongodb::{Client as MongoClient};
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::model::application::interaction::InteractionResponseType;
    use crate::{new_edition_insertion};

    pub async fn ping_reactor(command: ApplicationCommandInteraction, ctx: Context) {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Pong!"))
            })
            .await
            .expect("Failed to send interaction response");
    }

    pub async unsafe fn new_edition_reactor(client : &MongoClient, command : ApplicationCommandInteraction, context : Context) {
        let ctx = &context.clone();
        let com = &command.clone();
        new_edition_insertion(client, com).await;
        command.create_interaction_response(ctx.clone().http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Edition ajoutée")
                })
        }).await.unwrap();
    }
}