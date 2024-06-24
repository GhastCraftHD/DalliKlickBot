package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;

import java.io.File;

public record DatabaseDalliKlick(String id, String subject, String path, Difficulty difficulty) {

    public DalliKlick asDalliKlick(){
        return new DalliKlick(
                this.subject,
                new File(this.path),
                difficulty
        );
    }

    @Override
    public String toString() {
        return String.format(
                "DatabaseDalliKlick(%s): Subject: %s, Path: %s, Difficulty: %s",
                id,
                subject,
                path,
                difficulty.prettyName()
        );
    }
}
