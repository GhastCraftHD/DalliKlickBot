package de.leghast.dalliklick.exception;

public class ImageSaveException extends Exception {

    public ImageSaveException() {
    }

    public ImageSaveException(String message) {
        super(message);
    }

    public ImageSaveException(String message, Throwable cause) {
        super(message, cause);
    }

    public ImageSaveException(Throwable cause) {
        super(cause);
    }

    public ImageSaveException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
