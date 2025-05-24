use crate::command::{CommandError, require_options};
use crate::error::AppError;
use crate::error::AppError::Command;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption,
    Permissions,
};
use serenity::builder::CreateCommand;
use tracing::{info};

pub fn register() -> CreateCommand {
    info!("Registering command: /create");
    CreateCommand::new("create")
        .description("Creates a new challenge")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "prize",
            "The prize that is being fought for",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "start",
            "Start the challenge immediately",
        ))
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), AppError> {
    info!("{} is executing /create", interaction.user.name);

    interaction.defer_ephemeral(&ctx.http).await?;

    if !require_options(&interaction.data.options, vec!["prize"]) {
        return Err(Command(CommandError::InvalidCommandOptions));
    }

    Ok(())
}
