package de.leghast.dalliklick.command;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.exception.ImageSaveException;
import de.leghast.dalliklick.exception.UploadException;
import de.leghast.dalliklick.game.Difficulty;
import de.leghast.dalliklick.holder.DalliKlick;
import de.leghast.dalliklick.holder.DatabaseDalliKlick;
import net.dv8tion.jda.api.EmbedBuilder;
import net.dv8tion.jda.api.entities.MessageEmbed;
import net.dv8tion.jda.api.events.interaction.command.SlashCommandInteractionEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.awt.*;
import java.nio.file.Path;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;

public class UploadCommand {

    private static final Logger LOGGER = LoggerFactory.getLogger(UploadCommand.class);

    public UploadCommand(SlashCommandInteractionEvent event) {
        event.deferReply(true).queue();

        LOGGER.info(String.format("/upload was triggered by %s", event.getUser().getEffectiveName()));

        if (checkArguments(event)) return;

        Optional<Difficulty> optionalDifficulty = Difficulty.getByPrettyName(event.getOption("difficulty").getAsString());

        if (checkDifficulty(event, optionalDifficulty)) return;

        Difficulty difficulty = optionalDifficulty.orElseThrow();

        if (checkImage(event)) return;

        CompletableFuture<Path> future = event.getOption("image")
                .getAsAttachment()
                .getProxy().downloadToPath();

        future.thenAccept(
                path -> handleUpload(event, difficulty, path)
        );

    }

    private static void handleUpload(SlashCommandInteractionEvent event, Difficulty difficulty, Path path) {

        try {
            DatabaseDalliKlick uploaded = DalliKlickBot.HANDLERS.uploadHandler().upload(
                    new DalliKlick(
                            event.getOption("subject").getAsString(),
                            path.toFile(),
                            difficulty
                    )
            );
            respondToSuccess(event, uploaded);
        } catch (ImageSaveException | UploadException e) {
            respondToFailure(event);
        }

    }

    private static void respondToFailure(SlashCommandInteractionEvent event) {
        event.getHook()
                .sendMessage("Dein Dalli klick konnte nicht hochgeladen werden")
                .queue();
    }

    private static void respondToSuccess(SlashCommandInteractionEvent event, DatabaseDalliKlick uploaded) {
        MessageEmbed embed = new EmbedBuilder()
                .setTitle(String.format("%s hat ein neues Dalli Klick hochgeladen", event.getUser().getEffectiveName()))
                .setDescription(String.format("Dalli Klick \"%s\" wurde hochgeladen", uploaded.subject()))
                .setColor(new Color(0, 186, 71))
                .addField("Schwierigkeit:", uploaded.difficulty().prettyGermanName(), true)
                .setFooter(uploaded.id()).build();

        event.getHook()
                .setEphemeral(false)
                .sendMessage("Dein Dalli Klick wurde hochgeladen").queue();

        event.getChannel().asTextChannel().sendMessageEmbeds(embed).queue();


    }

    private static boolean checkImage(SlashCommandInteractionEvent event) {
        if(!event.getOption("image").getAsAttachment().isImage()){
            event.getHook()
                    .sendMessage("Die angehängte Datei muss ein Bild sein")
                    .queue();
            return true;
        }
        return false;
    }

    private static boolean checkDifficulty(SlashCommandInteractionEvent event, Optional<Difficulty> optionalDifficulty) {
        if(optionalDifficulty.isEmpty()){
            event.getHook()
                    .sendMessage("Bitte gib eine gültige Schwierigkeit an")
                    .queue();
            return true;
        }
        return false;
    }

    private static boolean checkArguments(SlashCommandInteractionEvent event) {
        if(event.getOptions().size() != 3){
            event.getHook()
                    .sendMessage("Bitte gib alle geforderten Angaben zum Hochladen eines Dalli Klicks an")
                    .queue();
            return true;
        }
        return false;
    }

}
