package de.leghast.dalliklick.database;

import com.surrealdb.driver.AsyncSurrealDriver;

@FunctionalInterface
public interface AsyncDriverQuery<T> {
    T execute(AsyncSurrealDriver driver);
}
