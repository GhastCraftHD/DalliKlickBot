package de.leghast.dalliklick.command;

import net.dv8tion.jda.api.events.interaction.command.MessageContextInteractionEvent;
import net.dv8tion.jda.api.interactions.components.ActionRow;
import net.dv8tion.jda.api.interactions.components.text.TextInput;
import net.dv8tion.jda.api.interactions.components.text.TextInputStyle;
import net.dv8tion.jda.api.interactions.modals.Modal;

public class EditCommand {

    public EditCommand(MessageContextInteractionEvent event) {
        TextInput subject = TextInput.create("subject", "Lösung", TextInputStyle.SHORT)
                .setPlaceholder("Lösung dieses Dalli Klicks")
                .setRequiredRange(5, 75)
                .setValue("Hamburg")
                .build();

        Modal modal = Modal.create("edit", "Dalli Klick bearbeiten")
                .addComponents(ActionRow.of(subject))
                .build();

        event.replyModal(modal).queue();
    }
}
