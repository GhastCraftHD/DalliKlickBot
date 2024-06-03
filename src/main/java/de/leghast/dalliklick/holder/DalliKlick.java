package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;

import java.io.File;

public record DalliKlick(String subject, File imageFile, Difficulty difficulty) {

    @Override
    public String toString() {
        return String.format(
                "DalliKlick: Subject: %s, Difficulty: %s",
                subject,
                difficulty.prettyName()
        );
    }
}
