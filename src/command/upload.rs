
use serenity::all::{CommandInteraction, CommandOptionType, Permissions};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use tracing::info;

pub fn register() -> CreateCommand {
    info!("Registering command: /upload");
    CreateCommand::new("upload")
        .description("Uploads a new DalliKlick dataset")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(CreateCommandOption::new(
            CommandOptionType::Attachment,
            "image",
            "The image of the DalliKlick subject"
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "subject",
            "The right answer to the DalliKlick"
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "difficulty",
            "The difficulty of the DalliKlick"
            )
            .add_string_choice("Easy", "easy")
            .add_string_choice("Medium", "medium")
            .add_string_choice("Hard", "hard")
            .add_string_choice("Extreme", "extreme")
        )
}


pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    
    info!("{} is executing /upload", interaction.user.name);
    
    let _ = interaction.defer_ephemeral(&ctx.http).await;
}