// ⚠️ MIXED SCRIPT ATTACK - Should FLAG (high severity)

// Latin + Greek mixing (deceptive)
const variαble = "attack";  // α is Greek, rest is Latin
const pαypal = "fake";      // Looks like "paypal"

// Latin + Cyrillic mixing (more deceptive)
const pаypal = "attack";    // а is Cyrillic
const vаriable = "attack";  // а is Cyrillic

// More mixed script examples
const fаke = "deceptive";   // а is Cyrillic
const hоme = "attack";      // о is Cyrillic
const tеst = "attack";      // е is Cyrillic

// Mixed with numbers (still deceptive)
const usеr123 = "attack";   // е is Cyrillic
const pαssw0rd = "attack";  // α is Greek, 0 is Latin digit
