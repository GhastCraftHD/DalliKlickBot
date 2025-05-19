mod ping;
mod upload;
mod create;

use serenity::all::{CommandDataOption, CommandInteraction};
use serenity::builder::{CreateCommand, EditInteractionResponse};
use serenity::client::Context;
use serenity::model::id::GuildId;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Missing or invalid command arguments")]
    InvalidCommandOptions,
}

pub async fn register(ctx: &Context, guild_id: u64) {
    let guild = GuildId::new(guild_id);

    let _ = guild.set_commands(&ctx.http, get_commands()).await;
}

fn get_commands() -> Vec<CreateCommand> {
    vec![
        ping::register(),
        upload::register(),
        create::register(),
    ]
}

pub async fn handle(ctx: &Context, command: &CommandInteraction) {
    let result = match command.data.name.as_str() {
        "upload" => upload::run(ctx, command).await,
        "create" => create::run(ctx, command).await,
        _ => Ok(()),
    };

    if let Err(e) = result {
        error!("Error during /upload: {}", e);
        let _ = command.edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(format!("Upload failed: {}", e))
        ).await;
    }
}

pub fn require_options(options: &Vec<CommandDataOption>, required: Vec<&str>) -> bool {
    required.iter().all(|&option_name| {
        options.iter().any(|opt| opt.name == option_name)
    })
}