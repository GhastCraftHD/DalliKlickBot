package de.leghast.dalliklick.exception;

public class GuildNotFoundException extends Exception{

    public GuildNotFoundException() {
    }

    public GuildNotFoundException(String message) {
        super(message);
    }

    public GuildNotFoundException(String message, Throwable cause) {
        super(message, cause);
    }

    public GuildNotFoundException(Throwable cause) {
        super(cause);
    }


}
