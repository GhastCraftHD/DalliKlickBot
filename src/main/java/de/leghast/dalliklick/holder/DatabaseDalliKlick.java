package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;

public record DatabaseDalliKlick(String subject, String imagePath, Difficulty difficulty) {

    @Override
    public String toString() {
        return String.format(
                "DatabaseDalliKlick: Subject: %s, Path: %s, Difficulty: %s",
                subject,
                imagePath,
                difficulty.prettyName()
        );
    }
}
