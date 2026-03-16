// BiDi Attack Patterns - True Positives
// These contain BiDi override characters that should be detected

// U+202E (RIGHT-TO-LEFT OVERRIDE) attack
// This makes the code appear different from what it executes
const accessLevel = "user";//‮//‭admin

// U+202B (RIGHT-TO-LEFT EMBEDDING) attack  
let role = "guest";//‮//‬// Actually sets to admin

// U+202A (LEFT-TO-RIGHT EMBEDDING) in comment
//‮// This comment hides code execution
const password = "admin123";//‬

// U+2067 (ISOLATE) and U+2068 (FIRST STRONG ISOLATE)
const token = "abc";//⁦//⁧// Actually modifies the line

// U+202D (LEFT-TO-RIGHT OVERRIDE)
var isAdmin = false;//‭//‬// Visually appears as true

// Classic Trojan Source pattern
// The BiDi characters make "admin" appear in a comment
// but actually the code assigns admin role
let userRole = "user";//‮//‭"admin"

// Nested BiDi attack
const config = {
    level: "basic",//‮//‭"elevated"
    access: false//‮//‭true
};

// BiDi in string that affects display
const message = "Hello‮‮‮"; // Multiple RLO characters
