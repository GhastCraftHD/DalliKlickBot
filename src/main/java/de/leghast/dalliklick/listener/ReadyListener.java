package de.leghast.dalliklick.listener;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.exception.GuildNotFoundException;
import de.leghast.dalliklick.game.Difficulty;
import net.dv8tion.jda.api.entities.Guild;
import net.dv8tion.jda.api.entities.User;
import net.dv8tion.jda.api.events.session.ReadyEvent;
import net.dv8tion.jda.api.hooks.ListenerAdapter;
import net.dv8tion.jda.api.interactions.commands.Command;
import net.dv8tion.jda.api.interactions.commands.OptionType;
import net.dv8tion.jda.api.interactions.commands.build.Commands;
import net.dv8tion.jda.api.interactions.commands.build.OptionData;
import net.dv8tion.jda.api.interactions.commands.build.SlashCommandData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Arrays;
import java.util.List;

public class ReadyListener extends ListenerAdapter {

    private static final Logger LOGGER = LoggerFactory.getLogger(ReadyListener.class);

    @Override
    public void onReady(ReadyEvent event) {
        try {
            setupGuild(event);
        } catch (GuildNotFoundException e) {
            DalliKlickBot.EXIT.exit(e);
        }
        registerCommands();
    }

    private void setupGuild(ReadyEvent event) throws GuildNotFoundException {
        String guildID = DalliKlickBot.INSTANCE.toml().getString("specification.guild_id");
        Guild guild = event.getJDA().getGuildById(guildID);

        if(guild == null)
            throw new GuildNotFoundException(String.format(
                    "Could not find guild with ID \"%s\"",
                    guildID
            ));

        DalliKlickBot.INSTANCE.guild(guild);
        LOGGER.info(String.format("Set %s as the main guild for this bot instance", guild.getName()));

    }

    private void registerCommands(){
        Guild guild = DalliKlickBot.INSTANCE.guild();

        List<Command.Choice> uploadChoices = Arrays.stream(Difficulty.values())
                .map(diff -> new Command.Choice(diff.prettyName(), diff.prettyName()))
                .toList();

        List<SlashCommandData> commandData = List.of(
                Commands.slash("upload", "Upload a new DalliKlick to the database")
                        .addOption(OptionType.ATTACHMENT, "image", "The image of the Dalli Klick subject")
                        .addOption(OptionType.STRING, "subject", "The right answer to the Dalli Klick")
                        .addOptions(
                                new OptionData(OptionType.STRING, "difficulty", "The difficulty of the Dalli Klick")
                                        .addChoices(uploadChoices)
                        )
        );

        guild.updateCommands().addCommands(commandData).queue();
        commandData.forEach(data -> LOGGER.info(String.format("Registered command \"%s\" on main guild", data.getName())));
    }

}
