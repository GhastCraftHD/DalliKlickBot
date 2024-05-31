package de.leghast;

import io.github.cdimascio.dotenv.Dotenv;
import net.dv8tion.jda.api.OnlineStatus;
import net.dv8tion.jda.api.entities.Activity;
import net.dv8tion.jda.api.sharding.DefaultShardManagerBuilder;
import net.dv8tion.jda.api.sharding.ShardManager;


public class DalliKlickBot {

    public static DalliKlickBot INSTANCE;

    private final Dotenv config;
    private final ShardManager shardManager;


    public DalliKlickBot() {

        this.config = Dotenv.configure().ignoreIfMalformed().load();

        this.shardManager = DefaultShardManagerBuilder.createDefault(config.get("TOKEN"))
                .setStatus(OnlineStatus.ONLINE)
                .setActivity(Activity.customStatus("Schreibt den 42. Bugreport"))
                .build();

    }

    public ShardManager shardManager() {
        return shardManager;
    }

    public Dotenv config() {
        return config;
    }

    public static void main(String[] args) {
        INSTANCE = new DalliKlickBot();
    }
}