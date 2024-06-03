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

        return upload(new DatabaseDalliKlick(
                dalliKlick.subject(),
                dalliKlick.imageFile().getPath(),
                dalliKlick.difficulty()
        ));
    }

    private State upload(DatabaseDalliKlick dalliKlick){

        DatabaseDalliKlick executed = DalliKlickBot.INSTANCE.database().executeQuery(driver -> driver.create("dalli_klick", dalliKlick));

        return (executed != null) ? State.SUCCESS : State.ERROR;
    }

}
