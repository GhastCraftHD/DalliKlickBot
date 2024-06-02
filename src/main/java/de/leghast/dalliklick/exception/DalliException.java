package de.leghast.dalliklick.exception;

public class DalliException extends Exception{

    private int status = 1;

    public DalliException() {
    }

    public DalliException(String message) {
        super(message);
    }

    public DalliException(String message, int status){
        this(message);
        this.status = status;
    }

    public DalliException(String message, Throwable cause) {
        super(message, cause);
    }

    public DalliException(Throwable cause) {
        super(cause);
    }

    public DalliException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }

    public int status() {
        return status;
    }
}
