package de.leghast.dalliklick.database;

import de.leghast.dalliklick.DalliKlickBot;

public record DatabaseCredentials(String user, String pass, String namespace, String database) {

    public DatabaseCredentials() {
       this(
            DalliKlickBot.INSTANCE.toml().getString("database.user"),
            DalliKlickBot.INSTANCE.toml().getString("database.pass"),
            DalliKlickBot.INSTANCE.toml().getString("database.namespace"),
            DalliKlickBot.INSTANCE.toml().getString("database.database")
       );
    }
}
