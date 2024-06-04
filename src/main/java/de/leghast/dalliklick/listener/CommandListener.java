package de.leghast.dalliklick.listener;

import de.leghast.dalliklick.command.UploadCommand;
import net.dv8tion.jda.api.events.interaction.command.SlashCommandInteractionEvent;
import net.dv8tion.jda.api.hooks.ListenerAdapter;
import org.jetbrains.annotations.NotNull;

import java.util.concurrent.CompletableFuture;

public class CommandListener extends ListenerAdapter {

    @Override
    public void onSlashCommandInteraction(@NotNull SlashCommandInteractionEvent event) {
        CompletableFuture.runAsync(() -> {
            String command = event.getName().toLowerCase();

            switch(command){
                case "upload" -> new UploadCommand(event);
            }
        });
    }
}
