// Homoglyph Attacks in JavaScript - True Positives
// Mixed script identifiers that should be detected

// Cyrillic 'а' (U+0430) looks like Latin 'a'
const аpi = "fake_api";  // Looks like "api"

// Cyrillic 'е' (U+0435) looks like Latin 'e'  
const sеcret = "exposed";  // Looks like "secret"

// Greek 'ο' (U+03BF) looks like Latin 'o'
const tοken = "stolen";  // Looks like "token"

// Mixed: Latin + Cyrillic + Greek
const аuth_еndpοint = "/hacked";

// Function name with homoglyph
function lоgin(user) {  // Cyrillic 'о'
    return authenticate(user);
}

// Class name with homoglyph
class Αdmin {  // Greek 'Α' (Alpha) looks like Latin 'A'
    constructor() {
        this.rоle = "admin";  // Cyrillic 'о'
    }
}

// Property access with homoglyph
const obj = { аdmin: true };  // Cyrillic 'а'
console.log(obj.аdmin);

// Import with homoglyph in module name
// impоrt { auth } from './auth';  // Cyrillic 'о'
