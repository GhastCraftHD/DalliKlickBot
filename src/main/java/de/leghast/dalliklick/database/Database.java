package de.leghast.dalliklick.database;

import com.surrealdb.connection.SurrealWebSocketConnection;
import com.surrealdb.driver.SyncSurrealDriver;

public class Database {

    private final SurrealWebSocketConnection connection;
    private final SyncSurrealDriver driver;

    private final DatabaseCredentials credentials;

    public Database(){
        this.connection = new SurrealWebSocketConnection("localhost", 3306, false);
        this.driver = new SyncSurrealDriver(connection);
        this.credentials = new DatabaseCredentials();
    }

    private void connect(){
        disconnect();

        connection.connect(5);
        driver.signIn(credentials.user(), credentials.pass());
        driver.use(credentials.namespace(), credentials.database());
    }

    private boolean isConnected(){
        return connection != null;
    }

    private void disconnect(){
        if(isConnected()) connection.disconnect();
    }

    public <T> T executeQuery(DriverQuery<T> operation){
        connect();
        T result = operation.execute(driver);
        disconnect();
        return result;
    }

}
