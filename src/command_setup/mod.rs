pub mod command_setup {
    use serenity::client::Context;
    use serenity::model::application::command::{Command, CommandOptionType};

    pub async fn ping_setup(ctx: &Context) {
        Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Reply with Pong")
        })
            .await.expect("Creation of ping command failed : ");
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
}