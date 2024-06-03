package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.holder.DalliKlick;
import de.leghast.dalliklick.holder.DatabaseDalliKlick;
import de.leghast.dalliklick.state.State;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Optional;

public class UploadHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(UploadHandler.class);

    public State upload(DalliKlick dalliKlick){
        FileHandler fileHandler = DalliKlickBot.HANDLERS.fileHandler();
        Optional<String> optionalPath = fileHandler.saveImage(dalliKlick.imageFile());
        if(optionalPath.isEmpty()) return State.ERROR;
        upload(new DatabaseDalliKlick(
                dalliKlick.subject(),
                optionalPath.orElseThrow(),
                dalliKlick.difficulty()
        ));
        return State.SUCCESS;
    }

    private void upload(DatabaseDalliKlick dalliKlick){
        DalliKlickBot.INSTANCE.database()
                .executeQuery(driver -> driver.create("dalli_klick", dalliKlick));
        LOGGER.info("Uploaded Dalli Klick to database");
        LOGGER.info(dalliKlick.toString());
    }

}
