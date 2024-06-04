package de.leghast.dalliklick;

import com.moandjiezana.toml.Toml;
import de.leghast.dalliklick.database.Database;
import de.leghast.dalliklick.handler.ExitHandler;
import de.leghast.dalliklick.holder.HandlerHolder;
import de.leghast.dalliklick.listener.CommandListener;
import de.leghast.dalliklick.listener.ContextListener;
import de.leghast.dalliklick.listener.ReadyListener;
import net.dv8tion.jda.api.JDA;
import net.dv8tion.jda.api.JDABuilder;
import net.dv8tion.jda.api.OnlineStatus;
import net.dv8tion.jda.api.entities.Activity;
import net.dv8tion.jda.api.entities.Guild;
import net.dv8tion.jda.api.requests.GatewayIntent;
import net.dv8tion.jda.api.utils.ChunkingFilter;
import net.dv8tion.jda.api.utils.MemberCachePolicy;
import net.dv8tion.jda.api.utils.cache.CacheFlag;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Paths;
import java.util.List;

public class DalliKlickBot {

    public static DalliKlickBot INSTANCE;
    public static final HandlerHolder HANDLERS = new HandlerHolder();
    private static final Logger LOGGER = LoggerFactory.getLogger(DalliKlickBot.class);
    public static final ExitHandler EXIT = new ExitHandler();

    private Toml toml;
    private JDA jda;
    private Guild guild;
    private Database database;

    public DalliKlickBot() {
        LOGGER.info("Starting up DalliKlick Bot");

        try {
            setupConfig();
            setupJDA();
        }catch (Exception e){
            EXIT.exit(e, 1);
        }

        registerListeners();
    }

    private void setupConfig() throws URISyntaxException {
        URL resource = DalliKlickBot.class.getClassLoader().getResource("config.toml");
        this.toml = new Toml().read(Paths.get(resource.toURI()).toFile());
        LOGGER.info("Successfully loaded config.toml");
    }

    private void setupJDA() throws Exception{
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
        LOGGER.info("Successfully connected Bot to Discord API");
    }

    private void registerListeners(){
        List.of(
            new ReadyListener(),
            new CommandListener(),
            new ContextListener()
        ).forEach(
                adapter -> {
                    jda.addEventListener(adapter);
                    LOGGER.info(String.format("Registered %s", adapter.getClass().getName()));
                }
        );
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

    public Database database() {
        return database;
    }

    public void database(Database database){
        this.database = database;
    }

    public static void main(String[] args) {
        INSTANCE = new DalliKlickBot();
    }
}