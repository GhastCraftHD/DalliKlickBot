package de.leghast.dalliklick.exception;

public class GuildNotFoundException extends DalliException{

    public GuildNotFoundException() {
    }

    public GuildNotFoundException(String message) {
        super(message);
    }

    public GuildNotFoundException(String message, int status) {
        super(message, status);
    }

    public GuildNotFoundException(String message, Throwable cause) {
        super(message, cause);
    }

    public GuildNotFoundException(Throwable cause) {
        super(cause);
    }

    public GuildNotFoundException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
