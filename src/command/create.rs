
use serenity::all::{CommandInteraction, Permissions};
use serenity::builder::CreateCommand;
use serenity::client::Context;
use tracing::info;

pub fn register() -> CreateCommand {
    info!("Registering command: /create");
    CreateCommand::new("create")
        .description("Creates a new DalliKlick")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

/*pub async fn run(ctx: &Context, command: &CommandInteraction) {
   let _ command.create_response(&ctx.http, )
}*/