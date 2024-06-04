package de.leghast.dalliklick.handler;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ExceptionHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(ExceptionHandler.class);

    public void handleCriticalException(Exception e){
        LOGGER.error("A critical error has occured! Shutting down", e);
        System.exit(1);
    }

}
