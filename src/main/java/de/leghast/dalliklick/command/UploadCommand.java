package de.leghast.dalliklick.command;

import net.dv8tion.jda.api.events.interaction.command.SlashCommandInteractionEvent;
import net.dv8tion.jda.api.interactions.commands.OptionMapping;

public class UploadCommand {

    public UploadCommand(SlashCommandInteractionEvent e) {
        
        for (OptionMapping option : e.getOptions()) {
            e.getHook().sendMessage(
                    String.format("%s: %s", option.getName(), option.getAsString())
            ).queue();
        }

    }
}
