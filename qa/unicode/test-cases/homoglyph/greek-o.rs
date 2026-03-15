// Homoglyph Attack Sample - Greek 'ο' vs Latin 'o'
// This file contains confusable characters for testing

// Variable with Greek 'ο' (U+03BF) instead of Latin 'o'
let lοgin = String::from("admin");  // Contains Greek ο

// Function with Greek characters
fn cοnnect() {  // Contains Greek ο
    println!("Connecting...");
}

// Multiple confusables
let hοst = "localhost";  // Greek ο
let pοrt = 8080;  // Greek ο
