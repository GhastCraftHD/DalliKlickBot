package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;
import org.apache.commons.lang3.RandomStringUtils;

import java.io.File;

public class DalliKlick {

    private final String id;
    private String subject;
    private File imageFile;
    private Difficulty difficulty;

    public DalliKlick(String subject, File imageFile, Difficulty difficulty) {
        this.id = RandomStringUtils.randomAlphanumeric(16);
        this.subject = subject;
        this.imageFile = imageFile;
        this.difficulty = difficulty;
    }

    public DalliKlick(String id, String subject, File imageFile, Difficulty difficulty) {
        this.id = id;
        this.subject = subject;
        this.imageFile = imageFile;
        this.difficulty = difficulty;
    }

    public String id() {
        return id;
    }

    public String subject() {
        return subject;
    }

    public void subject(String subject) {
        this.subject = subject;
    }

    public File imageFile() {
        return imageFile;
    }

    public void imageFile(File imageFile) {
        this.imageFile = imageFile;
    }

    public Difficulty difficulty() {
        return difficulty;
    }

    public void difficulty(Difficulty difficulty) {
        this.difficulty = difficulty;
    }

    public DatabaseDalliKlick asDatabaseDalliKlick(){
        return new DatabaseDalliKlick(
                id,
                subject,
                imageFile().getPath(),
                difficulty
        );
    }

    @Override
    public String toString() {
        return String.format(
                "DalliKlick(%s): Subject: %s, Difficulty: %s",
                id,
                subject,
                difficulty.prettyName()
        );
    }

    public static DalliKlick fromDatabaseDalliKlick(DatabaseDalliKlick dalliKlick){
        return new DalliKlick(
                dalliKlick.id(),
                dalliKlick.subject(),
                new File(dalliKlick.path()),
                dalliKlick.difficulty()
        );
    }
}
