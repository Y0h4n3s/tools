import java.util.regex.*;

public class PatternCheck {

    public static void main(String[] args) {
        if (args.length < 1) {
            System.out.println("Usage: java PatternCheck <pattern>");
            System.exit(0);
        }

        String pattern = args[0];
        try {

            Pattern.compile(pattern);
            System.out.println("Pattern Works For Java");
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }

}
