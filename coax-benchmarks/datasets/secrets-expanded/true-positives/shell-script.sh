#!/bin/bash
# Deployment script with exported credentials
# WARNING: Test secrets only

export AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE9"
export AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY9"
export GITHUB_TOKEN="ghp_1234567890abcdefghij1234567890ABCDEF"
export DATABASE_URL="postgresql://admin:ShellScriptPassword123@db.example.com:5432/prod"

echo "Deploying with AWS credentials..."
