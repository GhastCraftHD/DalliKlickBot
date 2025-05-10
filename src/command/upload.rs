use crate::command::check_options;
use crate::game::Difficulty;
use crate::holder::HolderKey;
use crate::io::upload::{UploadMetaDataBuilder};
use crate::{database};
use serenity::all::{
    Attachment, CommandData, CommandInteraction, CommandOptionType, Permissions,
};
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::client::Context;
use std::str::FromStr;
use tracing::info;
use crate::database::DatabaseRecord;

pub struct UploadOptions {
    subject: String,
    attachment: Attachment,
    difficulty: Difficulty,
}

impl UploadOptions {
    pub fn from_options(data: &CommandData) -> Self {
        let mut subject = None;
        let mut attachment = None;
        let mut difficulty = None;

        for option in &data.options {
            match option.name.as_str() {
                "subject" => {
                    if let Some(value) = option.value.as_str() {
                        subject = Some(value.to_string());
                    }
                }
                "image" => {
                    if let Some(value) = option.value.as_attachment_id() {
                        attachment = Some(data.resolved.attachments.get(&value).unwrap().clone());
                    }
                }
                "difficulty" => {
                    if let Some(value) = option.value.as_str() {
                        difficulty = Some(
                            Difficulty::from_str(value).expect("The difficulty of the Dalli Klick"),
                        );
                    }
                }
                _ => {}
            }
        }

        Self {
            subject: subject.expect("The subject of the Dalli Klick"),
            attachment: attachment.expect("The image attached to the Dalli Klick"),
            difficulty: difficulty.expect("The difficulty of the Dalli Klick"),
        }
    }
}

pub fn register() -> CreateCommand {
    info!("Registering command: /upload");
    CreateCommand::new("upload")
        .description("Uploads a new DalliKlick dataset")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(CreateCommandOption::new(
            CommandOptionType::Attachment,
            "image",
            "The image of the DalliKlick subject",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "subject",
            "The right answer to the DalliKlick",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "difficulty",
                "The difficulty of the DalliKlick",
            )
            .add_string_choice("Easy", "easy")
            .add_string_choice("Medium", "medium")
            .add_string_choice("Hard", "hard")
            .add_string_choice("Extreme", "extreme"),
        )
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    info!("{} is executing /upload", interaction.user.name);

    let _ = interaction.defer_ephemeral(&ctx.http).await;

    if !check_options(
        &interaction.data.options,
        vec!["image", "subject", "difficulty"],
    ) {
        info!("Command options for /upload are invalid");

        let _ = interaction
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Invalid command arguments!"),
            ).await;
    }

    let options = UploadOptions::from_options(&interaction.data);

    let meta_data = UploadMetaDataBuilder::new()
        .subject(options.subject)
        .file(options.attachment)
        .await
        .expect("Image file to be downloaded and stored")
        .difficulty(options.difficulty)
        .build();

    let data = ctx.data.read().await;

    if let Some(holder) = data.get::<HolderKey>() {
        let _: Option<DatabaseRecord> = database::connect(&holder.config.database).await
            .create(("dalliklick", &meta_data.id.to_string()))
            .content(meta_data.clone())
            .await
            .expect("Unexpected response from database");

        info!("{} uploaded Dalli Klick {}",&interaction.user.name, meta_data.id);

        let _ = interaction.edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .content(format!("{} uploaded DalliKlick with subject '{}'",&interaction.user.name ,meta_data.subject))
        ).await;

    } else {
        let _ = interaction.edit_response(
            &ctx.http,
            EditInteractionResponse::new().content("Error while accessing database credentials")
        ).await;

    }
}
