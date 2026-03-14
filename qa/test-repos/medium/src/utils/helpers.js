/**
 * Utility Functions
 * Clean code - no secrets.
 */

/**
 * Format a date
 */
function formatDate(date) {
    return date.toISOString().split('T')[0];
}

/**
 * Generate a random ID
 */
function generateId() {
    return Math.random().toString(36).substring(2, 15);
}

/**
 * Debounce a function
 */
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

/**
 * Deep clone an object
 */
function deepClone(obj) {
    return JSON.parse(JSON.stringify(obj));
}

/**
 * Check if value is empty
 */
function isEmpty(value) {
    if (value == null) return true;
    if (typeof value === 'string') return value.trim() === '';
    if (Array.isArray(value)) return value.length === 0;
    if (typeof value === 'object') return Object.keys(value).length === 0;
    return false;
}

module.exports = {
    formatDate,
    generateId,
    debounce,
    deepClone,
    isEmpty
};
