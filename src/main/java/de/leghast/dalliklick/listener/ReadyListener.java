package de.leghast.dalliklick.listener;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.exception.GuildNotFoundException;
import de.leghast.dalliklick.holder.BotCommands;
import net.dv8tion.jda.api.entities.Guild;
import net.dv8tion.jda.api.events.session.ReadyEvent;
import net.dv8tion.jda.api.hooks.ListenerAdapter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

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


        guild.updateCommands().addCommands(BotCommands.COMMAND_DATA).queue();
        BotCommands.COMMAND_DATA.forEach(
                data -> LOGGER.info(String.format("Registered command \"%s\" on main guild", data.getName())));
    }

}
