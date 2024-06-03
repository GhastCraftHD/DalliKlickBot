package de.leghast.dalliklick.state;

public enum State {
    SUCCESS,
    ERROR;

    public void ifSuccessfulOrElse(Runnable succ, Runnable err){
        if(this == SUCCESS) succ.run();
        else err.run();
    }

}
