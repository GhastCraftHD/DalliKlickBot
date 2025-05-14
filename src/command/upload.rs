use crate::command::{require_options, CommandError};
use crate::database;
use crate::error::AppError;
use crate::error::AppError::{Command, Io, SharedDataAccessError};
use crate::game::{Difficulty, GameError};
use crate::holder::HolderKey;
use crate::io::upload::DatabaseMetaDataBuilder;
use crate::io::IoError::Upload;
use serenity::all::CommandInteraction;
use serenity::all::{Attachment, CommandData, CommandOptionType, Permissions};
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::client::Context;
use std::str::FromStr;
use tracing::{error, info};

pub struct UploadOptions {
    subject: String,
    attachment: Attachment,
    difficulty: Difficulty,
}

impl UploadOptions {
    pub fn from_options(data: &CommandData) -> Result<Self, GameError> {
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
                            Difficulty::from_str(value)?,
                        );
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            subject: subject.expect("The subject of the Dalli Klick"),
            attachment: attachment.expect("The image attached to the Dalli Klick"),
            difficulty: difficulty.expect("The difficulty of the Dalli Klick"),
        })
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

pub(crate) async fn run(ctx: &Context, interaction: &CommandInteraction) {
    info!("{} is executing /upload", interaction.user.name);

    let _ = interaction.defer_ephemeral(&ctx.http).await;

    let result = process_upload(ctx, interaction).await;
    
    if let Err(e) = result {
        error!("Error during /upload: {}", e);
        let _ = interaction.edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(format!("Upload failed: {}", e))
        ).await;
    }
}

async fn process_upload(ctx: &Context, interaction: &CommandInteraction) -> Result<(), AppError> {
    if !require_options(
        &interaction.data.options,
        vec!["image", "subject", "difficulty"],
    ) {
        return Err(Command(CommandError::InvalidCommandOptions));
    }

    let options = UploadOptions::from_options(&interaction.data)?;

    let meta_data = DatabaseMetaDataBuilder::new()
        .subject(options.subject)
        .file(options.attachment)
        .await.map_err(|e| Io(Upload(e)))?
        .difficulty(options.difficulty)
        .build().map_err(|e| Io(Upload(e)))?;

    let data = ctx.data.read().await;
    let holder = data.get::<HolderKey>().ok_or(SharedDataAccessError)?;

    database::upload::upload_data(&holder.config.database, &meta_data).await?;
    
    info!(
        "{} uploaded Dalli Klick {}",
        &interaction.user.name,
        meta_data.id
    );
    
    interaction.channel_id.say(
        &ctx.http,
        format!(
            "<@{}> uploaded a DalliKlick with the subject '{}'",
            &interaction.user.id, 
            meta_data.subject
        )
    ).await.map_err(|e| AppError::Other(format!("Failed to send message: {}", e)))?;
    
    interaction.edit_response(
        &ctx.http,
        EditInteractionResponse::new().content("Upload successful!")
    ).await.map_err(|e| AppError::Other(format!("Failed to edit response: {}", e)))?;
    
    Ok(())
}
