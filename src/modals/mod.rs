pub mod modals {
    use serenity::builder::CreateEmbed;
    use serenity::model::application::component::ActionRowComponent;
    use serenity::model::application::interaction::InteractionResponseType;
    use serenity::model::application::interaction::modal::ModalSubmitInteraction;

    pub async fn new_edition_modal(mci : ModalSubmitInteraction, ctx : serenity::client::Context) {
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
}