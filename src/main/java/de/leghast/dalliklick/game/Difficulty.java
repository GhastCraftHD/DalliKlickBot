package de.leghast.dalliklick.game;

import java.util.Optional;

public enum Difficulty {

    VERY_EASY("Very Easy"),
    EASY("Easy"),
    NORMAL("Normal"),
    HARD("Hard"),
    VERY_HARD("Very Hard");

    final String prettyName;

    Difficulty(String prettyName){
        this.prettyName = prettyName;
    }

    public String prettyName(){
        return this.prettyName;
    }

    public static Optional<Difficulty> getByPrettyName(String prettyName){
        for (Difficulty difficulty : Difficulty.values()) {
            if(difficulty.prettyName.equals(prettyName)) return Optional.of(difficulty);
        }
        return Optional.empty();
    }

}
