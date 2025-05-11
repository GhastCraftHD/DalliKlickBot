use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommandOption, EditInteractionResponse, Permissions};
use serenity::builder::CreateCommand;
use tracing::info;
use crate::command::require_options;

pub fn register() -> CreateCommand {
    info!("Registering command: /create");
    CreateCommand::new("create")
        .description("Creates a new challenge")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "prize",
            "The prize that is being fought for"
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "start",
            "Start the challenge immediately"
        ))
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    info!("{} is executing /create", interaction.user.name);
    
    if !require_options(
        &interaction.data.options,
        vec!["prize"]
    ) {
        info!("Command options for /upload are invalid");

        let _ = interaction
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Invalid command arguments!"),
            ).await;

        return;
    }
    
    
    
}