use std::path::PathBuf;
use crate::command::{require_options, CommandError};
use crate::{database, holder};
use crate::error::AppError;
use crate::error::AppError::{Command, Io};
use crate::game::{Difficulty, GameError};
use crate::io::upload::DatabaseMetaDataBuilder;
use crate::io::IoError::Upload;
use serenity::all::CommandInteraction;
use serenity::all::{Attachment, CommandData, CommandOptionType, Permissions};
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::client::Context;
use std::str::FromStr;
use tracing::{info};
use crate::config::DatabaseConfig;
use crate::database::DatabaseRecord;

pub struct UploadOptions {
    subject: String,
    attachment: Attachment,
    difficulty: Difficulty,
}

struct UploadGuard {
    file_path: Option<PathBuf>,
    db_id: Option<DatabaseRecord>,
    db_config: Option<DatabaseConfig>,
    completed: bool,
}

impl UploadGuard {
    fn new() -> Self {
        Self {
            file_path: None,
            db_id: None,
            db_config: None,
            completed: false,
        }
    }

    fn mark_completed(mut self) {
        self.completed = true;
    }
}

impl Drop for UploadGuard {
    fn drop(&mut self) {
        if !self.completed {
            let file_path = self.file_path.clone();
            let db_id = self.db_id.clone();
            let db_config = self.db_config.clone();

            tokio::spawn(async move {
                if let Some(path) = file_path {
                    info!(
                        "Removing downloaded image {:?} since upload couldn't be completed", 
                        path.to_str()
                    );
                    let _ = tokio::fs::remove_file(path).await;
                }
                if let (Some(id), Some(config)) = (db_id, db_config) {
                    info!(
                        "Removing database record {} since upload couldn't be completed",
                        id.id
                    );
                    //TODO: implement database delete function
                    todo!("database delete function not implemented")
                }
            });
        }
    }
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

pub(crate) async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), AppError> {
    info!("{} is executing /upload", interaction.user.name);

    let mut guard = UploadGuard::new();

    interaction.defer_ephemeral(&ctx.http).await?;

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
                  
    guard.file_path = Some(meta_data.path.clone());
            
    let holder = holder::retrieve_holder(ctx).await?;

    guard.db_config = Some(holder.config.database.clone());
    guard.db_id = database::upload::upload_data(&holder.config.database, &meta_data).await?;

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

    guard.mark_completed();
    Ok(())
}