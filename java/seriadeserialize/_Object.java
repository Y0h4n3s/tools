package seriadeserialize;

import java.io.Serializable;

public class _Object implements Serializable {
    int num;
    String word;

    public _Object(int num, String word) {
        this.num = num;
        this.word = word;
    }

    @Override
    public String toString() {
        return "num: " + num + "  " + "word: " + word;
    }
}
