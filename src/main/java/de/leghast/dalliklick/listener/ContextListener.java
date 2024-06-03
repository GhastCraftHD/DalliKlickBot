package de.leghast.dalliklick.listener;

import de.leghast.dalliklick.command.EditCommand;
import net.dv8tion.jda.api.events.interaction.command.MessageContextInteractionEvent;
import net.dv8tion.jda.api.hooks.ListenerAdapter;

public class ContextListener extends ListenerAdapter {

    @Override
    public void onMessageContextInteraction(MessageContextInteractionEvent event) {
        String action = event.getName();

        switch (action){
            case "Dalli Klick bearbeiten" -> new EditCommand(event);
        }
    }
}
