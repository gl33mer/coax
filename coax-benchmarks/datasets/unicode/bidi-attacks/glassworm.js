// BiDi Glassworm Pattern - True Positives
// Glassworm uses BiDi to hide malicious code in plain sight

// The code appears to check for admin but actually grants admin
function checkAccess(user) {
    return user.role === "admin"; //‮//‭return true;
}

// BiDi makes this look like a comment but it's executed
const result = false; //‮//‭const result = true;

// Multiple BiDi overrides in sequence
let access = "denied"; //‮‮‮//‭‭‭"granted"

// BiDi in object property
const config = {
    admin: false,  //‮//‭true
    role: "user",  //‮//‭"admin"
    level: 1       //‮//‭99
};

// BiDi in array that changes meaning
const roles = ["user"]; //‮//‭["admin", "superuser"]

// Glassworm: code that looks safe but isn't
function validate(token) {
    return token.length > 10; //‮//‭return true;
}

// BiDi in return statement
function getAccess() {
    return "none"; //‮//‭"full"
}
