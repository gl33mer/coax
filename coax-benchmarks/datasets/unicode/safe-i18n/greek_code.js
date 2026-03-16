// Greek Variables and Comments - Safe i18n
// Legitimate Greek text in code - should NOT be flagged

// Greek variable names (common in Greek projects)
const τιμή = 100;  // "timi" = value/price
const όνομα = "Maria";  // "onoma" = name
const ηλικία = 30;  // "ilikia" = age

// Greek function names
function υπολογισμός() {  // "ypologismos" = calculation
    return τιμή * 2;
}

// Greek comments explaining code
// Αυτή η συνάρτηση υπολογίζει την τιμή
// (This function calculates the value)

// Mixed Greek and Latin (common in i18n projects)
const user_όνομα = "John";
const product_τιμή = 50.00;

// Greek string literals
const greeting = "Γεια σου κόσμε!";  // "Hello world!"
const message = "Καλημέρα!";  // "Good morning!"

// Greek in object properties
const ελληνικά = {
    καλημέρα: "Good morning",
    καλησπέρα: "Good evening",
    καληνύχτα: "Good night"
};

// Greek class names
class Χρήστης {  // "Christis" = User
    constructor(όνομα, ηλικία) {
        this.όνομα = όνομα;
        this.ηλικία = ηλικία;
    }
}
