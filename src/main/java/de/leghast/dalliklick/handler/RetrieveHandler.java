package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.holder.DalliKlick;
import de.leghast.dalliklick.holder.DatabaseDalliKlick;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RetrieveHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(RetrieveHandler.class);

    public DatabaseDalliKlick retrieveFromDatabase(String id){
        return DalliKlickBot.INSTANCE.database().executeQuery(
                driver -> driver.select(id, DatabaseDalliKlick.class)
        ).getFirst();
    }

    public DalliKlick retrieve(String id){
        return retrieveFromDatabase(id).asDalliKlick();
    }

}
