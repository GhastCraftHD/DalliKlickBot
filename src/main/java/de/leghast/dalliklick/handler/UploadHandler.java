package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.DalliKlickBot;
import de.leghast.dalliklick.exception.ImageSaveException;
import de.leghast.dalliklick.exception.UploadException;
import de.leghast.dalliklick.holder.DalliKlick;
import de.leghast.dalliklick.holder.DatabaseDalliKlick;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class UploadHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(UploadHandler.class);

    public void upload(DalliKlick dalliKlick) throws ImageSaveException, UploadException {
        FileHandler fileHandler = DalliKlickBot.HANDLERS.fileHandler();
        try {
            fileHandler.saveImage(dalliKlick.imageFile());
        } catch (IOException e) {
            LOGGER.error("Unable to save image to application directory", e);
            throw new ImageSaveException("Unable to save image to application directory", e);
        }

        upload(dalliKlick.asDatabaseDalliKlick());
    }

    private void upload(DatabaseDalliKlick dalliKlick) throws UploadException {
        LOGGER.info(String.format("Uploading %s", dalliKlick.toString()));
        DatabaseDalliKlick executed = DalliKlickBot.INSTANCE.database().executeQuery(driver -> driver.create("dalli_klick", dalliKlick));

        if(executed == null){
            LOGGER.error("An error occured while uploading DalliKlick to database");
            throw new UploadException("An error occured while uploading DalliKlick to database");
        }

        LOGGER.info("Successfully uploaded DalliKlick to database");
        LOGGER.info(dalliKlick.toString());
    }

}
