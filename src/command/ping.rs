use serenity::all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;
use serenity::client::Context;
use tracing::info;

pub fn register() -> CreateCommand {
    info!("Registering command: /ping");
    CreateCommand::new("ping").description("Responds with pong")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    info!("{} is executing command /ping", interaction.user.name);
    let _ = interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Pong!")
    )).await;
}