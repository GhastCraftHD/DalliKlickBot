mod ping;
mod upload;

use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::id::GuildId;

pub async fn register(ctx: &Context, guild_id: u64) {
    let guild = GuildId::new(guild_id);

    let _ = guild.set_commands(&ctx.http, get_commands()).await;
}

fn get_commands() -> Vec<CreateCommand> {
    vec![
        ping::register(),
        upload::register(),
    ]
}

pub async fn handle(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "ping" => ping::run(ctx, command).await,
        "upload" => upload::run(ctx, command).await,
        _ => {},
    }
}
