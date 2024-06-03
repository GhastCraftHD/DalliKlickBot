package de.leghast.dalliklick.game;

public enum Difficulty {

    VERY_EASY("Very Easy"),
    EASY("Easy"),
    NORMAL("Normal"),
    HARD("Hard"),
    VERY_HARD("Very Hard");

    String prettyName;

    Difficulty(String prettyName){
        this.prettyName = prettyName;
    }

    public String prettyName(){
        return this.prettyName;
    }

}
