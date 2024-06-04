package de.leghast.dalliklick.command;

import net.dv8tion.jda.api.EmbedBuilder;
import net.dv8tion.jda.api.entities.MessageEmbed;
import net.dv8tion.jda.api.events.interaction.command.SlashCommandInteractionEvent;

import java.awt.*;

public class DalliKlickCommand {

    public DalliKlickCommand(SlashCommandInteractionEvent event){
        MessageEmbed build = new EmbedBuilder()
                .setTitle("Dalli Klick Spielanleitung")
                .setDescription("Dalli Klick ist ein lustiges Ratespiel")
                .setColor(Color.RED)
                .addField("Field Title", "Field Content", false)
                .setFooter("Footer text")
                .build();

        event.replyEmbeds(build).queue();
    }

}
