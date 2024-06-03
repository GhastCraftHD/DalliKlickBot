package de.leghast.dalliklick.database;

import com.surrealdb.driver.SyncSurrealDriver;

@FunctionalInterface
public interface DriverQuery<T> {
    T execute(SyncSurrealDriver driver);
}
