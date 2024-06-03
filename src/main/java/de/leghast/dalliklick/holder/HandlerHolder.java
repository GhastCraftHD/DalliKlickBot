package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.handler.FileHandler;
import de.leghast.dalliklick.handler.UploadHandler;

public class HandlerHolder {

    private final UploadHandler uploadHandler;
    private final FileHandler fileHandler;

    public HandlerHolder() {
        this.uploadHandler = new UploadHandler();
        this.fileHandler = new FileHandler();
    }

    public UploadHandler uploadHandler() {
        return uploadHandler;
    }

    public FileHandler fileHandler() {
        return fileHandler;
    }
}
