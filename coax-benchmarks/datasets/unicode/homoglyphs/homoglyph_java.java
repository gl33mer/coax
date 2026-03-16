// Homoglyph Attacks in Java - True Positives
// Mixed script identifiers in Java code

public class HomoglyphAttack {
    // Cyrillic 'а' (U+0430) in variable name
    private String аpiKey = "stolen_key";  // Looks like "apiKey"
    
    // Greek 'Ι' (U+0399) in constant
    public static final String ΤOKEN = "fake";  // Greek 'Τ'
    
    // Mixed script method name
    public void lоgin() {  // Cyrillic 'о'
        System.out.println("Logging in");
    }
    
    // Cyrillic 'е' in parameter
    public void setUser(String usеr) {  // Looks like "user"
        // Process user
    }
    
    // Multiple homoglyphs
    private boolean аuthоrized = false;  // Cyrillic 'а' and 'о'
    
    // Greek 'ο' in return type context
    public String getTοken() {  // Greek 'ο'
        return tοken;
    }
    
    // Field that looks like keyword
    private int сlass = 10;  // Cyrillic 'с'
}
