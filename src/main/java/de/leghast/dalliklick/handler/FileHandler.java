package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.DalliKlickBot;
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
                DalliKlickBot.EXIT.exit(e, 2);
            }
        }
    }

    private void setupAppDir() {
        if(!Files.exists(appDir)){
            try {
                Files.createDirectories(appDir);
            } catch (IOException e) {
                LOGGER.error("Unable to access application directory");
                DalliKlickBot.EXIT.exit(e, 2);
            }
        }
    }

    public void saveImage(File imageFile) throws IOException{
        Path destinationPath = imageDir.resolve(imageFile.getName());
        Files.move(imageFile.toPath(), destinationPath);
        LOGGER.info(String.format(
                "Saved image %s to %s",
                imageFile.getName(),
                destinationPath
        ));
    }

}
