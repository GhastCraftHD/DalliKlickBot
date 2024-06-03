package de.leghast.dalliklick.handler;

import de.leghast.dalliklick.DalliKlickBot;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Optional;

public class FileHandler {

    private static final Logger LOGGER = LoggerFactory.getLogger(FileHandler.class);

    private final String homeDir = System.getProperty("user.home");
    Path appDir = Paths.get(homeDir, ".dalliklick");

    public FileHandler(){
        if(!Files.exists(appDir)){
            try {
                Files.createDirectories(appDir);
            } catch (IOException e) {
                LOGGER.error("Unable to access application directory");
                DalliKlickBot.EXIT.exit(e, 2);
            }
        }
    }

    public Optional<String> saveImage(File imageFile){
        try{
            Path destinationPath = appDir.resolve(imageFile.getName());
            Files.copy(imageFile.toPath(), destinationPath);
            return Optional.of(destinationPath.toString());
        }catch(IOException e){
            LOGGER.error("Unable to save image");
            return Optional.empty();
        }
    }

}
