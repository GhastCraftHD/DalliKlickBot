mod ping;
mod upload;
mod create;

use serenity::all::{CommandDataOption, CommandInteraction};
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::id::GuildId;
use thiserror::Error;

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
    match command.data.name.as_str() {
        "ping" => ping::run(ctx, command).await,
        "upload" => upload::run(ctx, command).await,
        "create" => create::run(ctx, command).await,
        _ => {},
    }
}

pub fn require_options(options: &Vec<CommandDataOption>, required: Vec<&str>) -> bool {
    required.iter().all(|&option_name| {
        options.iter().any(|opt| opt.name == option_name)
    })
}
