package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;

public record DatabaseDalliKlick(String id, String subject, String path, Difficulty difficulty) {

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
