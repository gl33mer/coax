#!/bin/bash
# Legacy Deployment Script
# Contains embedded secrets (bad practice!)
#
# WARNING: Contains intentional secrets for testing.

# SSH Configuration (CRITICAL - should be detected)
SSH_KEY="-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyF8PbnGy0AHB7MxUK
-----END RSA PRIVATE KEY-----"

# Server credentials
SERVER_USER="deploy"
SERVER_PASSWORD="DeployPassword123!"
SERVER_HOST="prod.example.com"

# GitHub Token for deployments (CRITICAL - should be detected)
GITHUB_TOKEN="ghp_abcdefghijklmnopqrstuvwxyz1234567890"

# Heroku API Key (CRITICAL - should be detected)
HEROKU_API_KEY="12345678-1234-1234-1234-123456789012"

# Deployment function
deploy() {
    echo "Deploying to $SERVER_HOST..."
    # In real script, this would use the credentials above
    echo "Using GitHub token: ${GITHUB_TOKEN:0:10}..."
}

# Backup function
backup() {
    echo "Creating backup..."
    # Would use SSH_KEY for secure connection
}

# Main
case "$1" in
    deploy)
        deploy
        ;;
    backup)
        backup
        ;;
    *)
        echo "Usage: $0 {deploy|backup}"
        exit 1
        ;;
esac

exit 0
