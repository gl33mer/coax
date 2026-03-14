/**
 * Payment Configuration Module
 * Handles payment processor integrations.
 * 
 * WARNING: This file contains intentional secrets for testing.
 */

// Stripe Configuration (CRITICAL - should be detected)
const STRIPE_SECRET_KEY = 'sk_live_abcdefghijklmnopqrstuvwxyz123456';
const STRIPE_WEBHOOK_SECRET = 'whsec_1234567890abcdefghijklmnop';

// Square Configuration (CRITICAL - should be detected)
const SQUARE_ACCESS_TOKEN = 'sq0atp-abcdefghijklmnopqrstuv';
const SQUARE_APPLICATION_ID = 'sq0idp-1234567890abcdef';

// PayPal Configuration (CRITICAL - should be detected)
const PAYPAL_CLIENT_ID = 'AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz';
const PAYPAL_SECRET = 'EeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1234567890';

/**
 * Initialize Stripe
 */
function initStripe() {
    console.log('Initializing Stripe...');
    // In production, use environment variables
    return {
        apiKey: STRIPE_SECRET_KEY,
        webhookSecret: STRIPE_WEBHOOK_SECRET
    };
}

/**
 * Initialize Square
 */
function initSquare() {
    console.log('Initializing Square...');
    return {
        accessToken: SQUARE_ACCESS_TOKEN,
        applicationId: SQUARE_APPLICATION_ID
    };
}

/**
 * Process a payment
 */
async function processPayment(amount, currency, source) {
    // Payment processing logic
    return {
        success: true,
        transactionId: `txn_${Date.now()}`,
        amount,
        currency
    };
}

/**
 * Refund a payment
 */
async function refundPayment(transactionId, amount) {
    // Refund logic
    return {
        success: true,
        refundId: `ref_${Date.now()}`,
        transactionId,
        amount
    };
}

module.exports = {
    initStripe,
    initSquare,
    processPayment,
    refundPayment,
    STRIPE_SECRET_KEY,
    SQUARE_ACCESS_TOKEN
};
