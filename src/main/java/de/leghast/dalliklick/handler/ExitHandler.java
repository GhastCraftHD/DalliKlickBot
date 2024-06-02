package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.exception.DalliException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ExitHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(ExitHandler.class);

    public void exit(DalliException e){
        LOGGER.error("Shutting down because of an internal error");
        LOGGER.error(e.getMessage());
        for (StackTraceElement element : e.getStackTrace()) {
            LOGGER.error(element.toString());
        }
        System.exit(e.status());
    }

    public void exit(Exception e, int status){
        exit(new DalliException(e.getMessage(), status));
    }

}
