# Homoglyph Attack Patterns - True Positives
# These use confusable characters from different scripts

# Mixed script identifier: Latin + Cyrillic
# The 'а' below is Cyrillic (U+0430), not Latin 'a' (U+0061)
const аdmin = "attacker";  // Looks like "admin" but uses Cyrillic а

# Mixed Latin and Greek
const pаssword = "hacked";  # Greek 'ρ' (rho) looks like Latin 'p'

# Cyrillic 'е' (U+0435) vs Latin 'e' (U+0061)
const usеr = "fake_user";  # Uses Cyrillic е

# Mixed Latin and Cyrillic in variable name
const authоrized = true;  # Cyrillic 'о' (U+043E) vs Latin 'o'

# Greek omicron 'ο' (U+03BF) vs Latin 'o'
const tоken = "stolen";  # Uses Greek omicron

# Multiple homoglyphs in one identifier
const аdmin_аuth = {  # Both 'а' are Cyrillic
    level: "full"
};

# Latin 'I' vs Cyrillic 'І' vs Greek 'Ι'
const ID = "confused";  # Could be any of the three

# Digit zero vs Latin 'O' vs Greek omicron
const zer0_O_ο = "ambiguous";
