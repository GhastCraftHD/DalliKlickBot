package de.leghast.dalliklick.database;

import com.moandjiezana.toml.Toml;
import com.surrealdb.connection.SurrealWebSocketConnection;
import com.surrealdb.driver.SyncSurrealDriver;
import de.leghast.dalliklick.DalliKlickBot;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Database {

    private static final Logger LOGGER = LoggerFactory.getLogger(Database.class);

    private SurrealWebSocketConnection connection;
    private SyncSurrealDriver driver;
    private final Toml toml;

    private final DatabaseCredentials credentials;

    public Database(){
        LOGGER.info("Initialising database");
        this.toml = DalliKlickBot.INSTANCE.toml();

        this.credentials = new DatabaseCredentials();
    }

    private void connect(){
        disconnect();
        LOGGER.info("Connecting to database...");

        this.connection = new SurrealWebSocketConnection(
                toml.getString("database.host"),
                Integer.parseInt(toml.getString("database.port")),
                false
        );

        connection.connect(5);
        LOGGER.info(String.format(
                "Connected to database on %s:%s",
                toml.getString("database.host"),
                toml.getString("database.port")
        ));

        this.driver = new SyncSurrealDriver(connection);
        driver.signIn(credentials.user(), credentials.pass());
        driver.use(credentials.namespace(), credentials.database());

    }

    private boolean isConnected(){
        return connection != null;
    }

    private void disconnect(){
        if(isConnected()) {
            connection.disconnect();
            LOGGER.info("Disconnected from database");
        }
    }

    public <T> T executeQuery(DriverQuery<T> query){
        connect();
        T result = query.execute(driver);
        disconnect();
        return result;
    }

}
