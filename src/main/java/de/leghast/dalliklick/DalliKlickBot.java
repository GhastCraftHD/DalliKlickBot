package de.leghast.dalliklick;

import com.moandjiezana.toml.Toml;
import de.leghast.dalliklick.handler.ExitHandler;
import de.leghast.dalliklick.listener.CommandListener;
import de.leghast.dalliklick.listener.ReadyListener;
import net.dv8tion.jda.api.JDA;
import net.dv8tion.jda.api.JDABuilder;
import net.dv8tion.jda.api.OnlineStatus;
import net.dv8tion.jda.api.entities.Activity;
import net.dv8tion.jda.api.entities.Guild;
import net.dv8tion.jda.api.hooks.ListenerAdapter;
import net.dv8tion.jda.api.requests.GatewayIntent;
import net.dv8tion.jda.api.utils.ChunkingFilter;
import net.dv8tion.jda.api.utils.MemberCachePolicy;
import net.dv8tion.jda.api.utils.cache.CacheFlag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.net.URL;
import java.nio.file.Paths;
import java.util.List;

public class DalliKlickBot {

    public static DalliKlickBot INSTANCE;
    public static final ExitHandler EXIT = new ExitHandler();

    private Toml toml;
    private JDA jda;
    private Guild guild;
    private static final Logger LOGGER = LoggerFactory.getLogger(DalliKlickBot.class);

    public DalliKlickBot() {
        try {
            URL resource = DalliKlickBot.class.getClassLoader().getResource("config.toml");
            this.toml = new Toml().read(Paths.get(resource.toURI()).toFile());

            this.jda = JDABuilder.createDefault(toml.getString("bot.token"))
                    .setStatus(OnlineStatus.ONLINE)
                    .setActivity(Activity.customStatus("Schreibt die 12. Teambeschwerde"))
                    .enableIntents(
                            GatewayIntent.GUILD_MEMBERS,
                            GatewayIntent.GUILD_MESSAGES,
                            GatewayIntent.GUILD_PRESENCES
                    )
                    .setMemberCachePolicy(MemberCachePolicy.ALL)
                    .setChunkingFilter(ChunkingFilter.ALL)
                    .enableCache(CacheFlag.ONLINE_STATUS)
                    .build();
        }catch (Exception e){
            EXIT.exit(e, 1);
        }

        registerListeners();
    }

    private void registerListeners(){
        List<ListenerAdapter> listenerAdapters = List.of(
            new ReadyListener(),
            new CommandListener()
        );

        for (ListenerAdapter adapter : listenerAdapters) {
            jda.addEventListener(adapter);
        }
    }

    public JDA jda(){
        return jda;
    }

    public Toml toml(){
        return toml;
    }

    public Guild guild() {
        return guild;
    }

    public void guild(Guild guild) {
        this.guild = guild;
    }

    public static void main(String[] args) {
        INSTANCE = new DalliKlickBot();
    }
}