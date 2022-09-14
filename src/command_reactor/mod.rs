pub mod command_reactor {
    use serenity::client::Context;
    use mongodb::{Client as MongoClient};
    use serenity::model::application::component::InputTextStyle;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::model::application::interaction::InteractionResponseType;
    use crate::{new_edition_insertion};

    pub async fn ping_reactor(command: &ApplicationCommandInteraction, ctx: &Context) {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Pong!"))
            })
            .await
            .expect("Failed to send interaction response");
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
}