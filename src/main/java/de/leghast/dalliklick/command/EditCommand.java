package de.leghast.dalliklick.command;

import net.dv8tion.jda.api.entities.Message;
import net.dv8tion.jda.api.events.interaction.command.MessageContextInteractionEvent;

import java.util.regex.Pattern;

public class EditCommand {

    private final Pattern dalliKlickId = Pattern.compile("dalli_klick:[A-Za-z0-9]{16}");

    public EditCommand(MessageContextInteractionEvent event) {
        event.deferReply(true).queue();

        Message target = event.getInteraction().getTarget();

    }

    public void responseNoDalliKlick(MessageContextInteractionEvent event){

    }

}
