/**
 * API Client Module
 * Handles communication with external AI services.
 * 
 * WARNING: This file contains intentional secrets for testing.
 */

const axios = require('axios');

// API Configuration
const API_TIMEOUT = 30000;
const MAX_RETRIES = 3;

// OpenAI Configuration (CRITICAL - should be detected)
const OPENAI_API_KEY = 'sk-proj-1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJ';
const OPENAI_BASE_URL = 'https://api.openai.com/v1';

// Anthropic Configuration (CRITICAL - should be detected)  
const ANTHROPIC_API_KEY = 'sk-ant-api01-abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGH';
const ANTHROPIC_BASE_URL = 'https://api.anthropic.com/v1';

/**
 * Create an OpenAI client
 */
function createOpenAIClient() {
    return axios.create({
        baseURL: OPENAI_BASE_URL,
        headers: {
            'Authorization': `Bearer ${OPENAI_API_KEY}`,
            'Content-Type': 'application/json'
        },
        timeout: API_TIMEOUT
    });
}

/**
 * Create an Anthropic client
 */
function createAnthropicClient() {
    return axios.create({
        baseURL: ANTHROPIC_BASE_URL,
        headers: {
            'X-API-Key': ANTHROPIC_API_KEY,
            'Content-Type': 'application/json'
        },
        timeout: API_TIMEOUT
    });
}

/**
 * Send a message to OpenAI
 */
async function sendToOpenAI(prompt, options = {}) {
    const client = createOpenAIClient();
    const response = await client.post('/chat/completions', {
        model: options.model || 'gpt-4',
        messages: [{ role: 'user', content: prompt }],
        ...options
    });
    return response.data;
}

/**
 * Send a message to Anthropic
 */
async function sendToAnthropic(prompt, options = {}) {
    const client = createAnthropicClient();
    const response = await client.post('/messages', {
        model: options.model || 'claude-3-opus-20240229',
        max_tokens: options.maxTokens || 1024,
        messages: [{ role: 'user', content: prompt }]
    });
    return response.data;
}

module.exports = {
    createOpenAIClient,
    createAnthropicClient,
    sendToOpenAI,
    sendToAnthropic,
    OPENAI_API_KEY,
    ANTHROPIC_API_KEY
};
