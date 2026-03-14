/**
 * Frontend API Client
 * JavaScript/React API integration
 * 
 * WARNING: Contains intentional secrets for testing.
 */

import axios from 'axios';

// API Configuration (CRITICAL - should be detected)
const OPENAI_API_KEY = 'sk-proj-abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJ';
const API_BASE_URL = 'https://api.example.com/v1';

// Discord Bot Token (CRITICAL - should be detected)
const DISCORD_BOT_TOKEN = 'MTIzNDU2Nzg5MDEyMzQ1Njc4.GaBcDe.FgHiJkLmNoPqRsTuVwXyZ1234567890AbCdEf';

const apiClient = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Authorization': `Bearer ${OPENAI_API_KEY}`,
        'Content-Type': 'application/json'
    }
});

export async function fetchUserData(userId) {
    const response = await apiClient.get(`/users/${userId}`);
    return response.data;
}

export async function sendMessage(content) {
    const response = await apiClient.post('/messages', { content });
    return response.data;
}

export { apiClient, OPENAI_API_KEY, DISCORD_BOT_TOKEN };
