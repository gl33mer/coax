// Zero-Width Characters in Code - True Positives
// Invisible characters that modify code behavior

// ZWSP (U+200B) splits identifier
const us‌er = "admin";  // Looks like "user" but has ZWSP

// ZWJ (U+200D) joins different characters
const admin‍role = "superuser";  // ZWJ after admin

// Zero-width in function name
function log‌in() {  // ZWSP in "login"
    return true;
}

// Invisible character in string comparison
if (password === "secret") {  // ZWSP in "secret"
    grantAccess();
}

// ZWSP in import path
// import { auth } from './a‌uth';  // ZWSP in path

// Multiple invisible chars
const t‌o‌k‌e‌n = "abc123";  // ZWSP between each letter

// Zero-width at line end (hard to detect visually)
const secret = "value";  // ZWSP at end

// Invisible in template literal
const query = `SELECT * FROM us‌ers`;  // ZWSP in "users"

// ZWJ in object property
const obj = { admin‍: true };  // ZWJ after admin
