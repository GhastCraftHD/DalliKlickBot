package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.exception.DirectoryException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class FileHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(FileHandler.class);

    private final String homeDir = System.getProperty("user.home");
    Path appDir = Paths.get(homeDir, ".dalliklick");
    Path imageDir = appDir.resolve("images");

    public FileHandler(){
        setupAppDir();

        setupImageDir();
    }

    private void setupImageDir() {
        if(!Files.exists(imageDir)){
            try {
                Files.createDirectories(imageDir);
            } catch (IOException e) {
                LOGGER.error("Unable to access image directory");
                new ExceptionHandler()
                        .handleCriticalException(
                                new DirectoryException("Unable to access image directory in application folder"));
            }
        }
    }

    private void setupAppDir() {
        if(!Files.exists(appDir)){
            try {
                Files.createDirectories(appDir);
            } catch (IOException e) {
                LOGGER.error("Unable to access application directory");
                new ExceptionHandler()
                        .handleCriticalException(new DirectoryException("Unable to access application directory"));
            }
        }
    }

    public File saveImage(File imageFile) throws IOException{
        Path destinationPath = imageDir.resolve(imageFile.getName());
        File moved = Files.move(imageFile.toPath(), destinationPath).toFile();
        LOGGER.info(String.format(
                "Saved image %s to %s",
                imageFile.getName(),
                moved.getPath()
        ));
        return moved;
    }

}
