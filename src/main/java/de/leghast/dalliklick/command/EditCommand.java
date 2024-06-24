package de.leghast.dalliklick.command;

import de.leghast.dalliklick.handler.RetrieveHandler;
import de.leghast.dalliklick.holder.DalliKlick;
import net.dv8tion.jda.api.entities.Message;
import net.dv8tion.jda.api.entities.MessageEmbed;
import net.dv8tion.jda.api.events.interaction.command.MessageContextInteractionEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.regex.Pattern;

public class EditCommand {

    private final Pattern dalliKlickId = Pattern.compile("dalli_klick:[A-Za-z0-9]{16}");

    private static final Logger LOGGER = LoggerFactory.getLogger(EditCommand.class);

    public EditCommand(MessageContextInteractionEvent event) {
        event.deferReply(true).queue();

        Message target = event.getInteraction().getTarget();

        String id = target.getEmbeds().getFirst().getFooter().getText();

        DalliKlick dalliKlick = new RetrieveHandler().retrieve(id);

        LOGGER.info(dalliKlick.toString());

    }

    public void responseNoDalliKlick(MessageContextInteractionEvent event){

    }

}
