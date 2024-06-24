package de.leghast.dalliklick.game;

import java.util.Optional;

public enum Difficulty {

    VERY_EASY("Very Easy", "Sehr einfach"),
    EASY("Easy", "Einfach"),
    NORMAL("Normal", "Normal"),
    HARD("Hard", "Schwer"),
    VERY_HARD("Very Hard", "Sehr schwer");

    final String prettyName;
    final String prettyGermanName;

    Difficulty(String prettyName, String prettyGermanName){
        this.prettyName = prettyName;
        this.prettyGermanName = prettyGermanName;
    }

    public String prettyName(){
        return this.prettyName;
    }

    public String prettyGermanName(){
        return this.prettyGermanName;
    }

    public static Optional<Difficulty> getByPrettyName(String prettyName){
        for (Difficulty difficulty : Difficulty.values()) {
            if(difficulty.prettyName.equals(prettyName)) return Optional.of(difficulty);
        }
        return Optional.empty();
    }

}
