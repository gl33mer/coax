#!/bin/bash
# Generate expanded benchmark corpus for Coax
# Creates 50+ TP and 30+ TN test files

TP_DIR="coax-benchmarks/datasets/secrets-expanded/true-positives"
TN_DIR="coax-benchmarks/datasets/secrets-expanded/true-negatives"

mkdir -p "$TP_DIR" "$TN_DIR"

echo "Creating expanded benchmark corpus..."

# ============================================================================
# TRUE POSITIVES (50+ files)
# ============================================================================

# Copy original 15 TP files
cp coax-benchmarks/datasets/secrets/true-positives/*.txt "$TP_DIR/" 2>/dev/null || true

# --- Python config files ---
cat > "$TP_DIR/python-config.py" << 'EOF'
# Python application configuration
# WARNING: This file contains test secrets only

import os

# AWS Credentials
AWS_ACCESS_KEY_ID = "AKIAIOSFODNN7EXAMPLE1"
AWS_SECRET_ACCESS_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"

# GitHub Token
GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890ABCDEF"

# Stripe API Key
STRIPE_SECRET_KEY = "sk_live_1234567890abcdefghijklmnopqrstuv"

# Database connection
DATABASE_URL = "postgresql://admin:SuperSecretPassword123@db.example.com:5432/production"

# SendGrid
SENDGRID_API_KEY = "SG.abcdefghijklmnopqrstuvwx.1234567890abcdefghijklmnopqrstuvwxyz12345"

# OpenAI
OPENAI_API_KEY = "sk-1234567890abcdefghijklmnopqrstuvwx1234567890ABCDEFGH"
EOF

# --- JavaScript/Node.js env file ---
cat > "$TP_DIR/javascript-env.js" << 'EOF'
// Node.js application environment config
// Test secrets only - do not use in production

module.exports = {
    // GitHub Personal Access Token
    github: {
        token: "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef12"
    },
    
    // NPM Registry Token
    npm: {
        token: "npm_1234567890abcdefghijklmnopqrstuvwx"
    },
    
    // Discord Bot Token
    discord: {
        token: "MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.AbCdEf.GhIjKlMnOpQrStUvWxYz1234567890"
    },
    
    // Telegram Bot Token
    telegram: {
        token: "1234567890:AbCdEfGhIjKlMnOpQrStUvWxYz1234567890"
    }
};
EOF

# --- Terraform variables ---
cat > "$TP_DIR/terraform-vars.tf" << 'EOF'
# Terraform variables with test secrets
# DO NOT COMMIT TO VERSION CONTROL

variable "aws_access_key" {
  default = "AKIAIOSFODNN7EXAMPLE2"
}

variable "aws_secret_key" {
  default = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY2"
}

variable "azure_client_secret" {
  default = "azureClientSecret1234567890abcdefghijklmnop"
}

variable "gcp_api_key" {
  default = "AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe"
}

variable "datadog_api_key" {
  default = "1234567890abcdef1234567890abcdef"
}
EOF

# --- Docker Compose ---
cat > "$TP_DIR/docker-compose.yml" << 'EOF'
version: '3.8'
services:
  app:
    image: myapp:latest
    environment:
      - AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE3
      - AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY3
      - DATABASE_PASSWORD=SuperSecretDbPassword123
      - REDIS_PASSWORD=RedisSecretPass456
      - JWT_SECRET=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.secret
      - SLACK_BOT_TOKEN=xoxb-123456789012-1234567890123-AbCdEfGhIjKlMnOpQrStUvWx
  
  db:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: SuperSecretPostgresPassword789
EOF

# --- .env file ---
cat > "$TP_DIR/dotenv-file.env" << 'EOF'
# Environment variables with test secrets
# DO NOT COMMIT

# AWS
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE4
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY4
AWS_SESSION_TOKEN=FwoGZXIvYXdzEBYaDH7example1234567890

# GitHub
GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890ABCDEF
GITHUB_OAUTH=gho_1234567890abcdefghij1234567890ABCDEF

# Stripe
STRIPE_SECRET_KEY=sk_live_1234567890abcdefghijklmnopqrstuv
STRIPE_TEST_KEY=sk_test_1234567890abcdefghijklmnopqrstuv

# Twilio
TWILIO_ACCOUNT_SID=AC1234567890abcdef1234567890abcdef
TWILIO_AUTH_TOKEN=1234567890abcdef1234567890abcdef

# Mailgun
MAILGUN_API_KEY=key-1234567890abcdefghijklmnopqrstuvwx

# Slack
SLACK_BOT_TOKEN=xoxb-123456789012-1234567890123-AbCdEfGhIjKlMnOpQrStUvWx
SLACK_USER_TOKEN=xoxp-123456789012-1234567890123-1234567890124-AbCdEfGhIjKlMnOpQrStUvWxYz

# Datadog
DD_API_KEY=1234567890abcdef1234567890abcdef
DD_APP_KEY=abcdef1234567890abcdef1234567890abcdef12

# Anthropic
ANTHROPIC_API_KEY=sk-ant-1234567890abcdefghijklmnopqrstuvwx1234567890ABCDEFGHIJ
EOF

# --- Kubernetes secret ---
cat > "$TP_DIR/kubernetes-secret.yaml" << 'EOF'
apiVersion: v1
kind: Secret
metadata:
  name: app-secrets
  namespace: production
type: Opaque
stringData:
  aws-access-key: "AKIAIOSFODNN7EXAMPLE5"
  aws-secret-key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY5"
  github-token: "ghp_1234567890abcdefghij1234567890ABCDEF"
  database-password: "KubernetesSecretPassword123"
  jwt-signing-key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signingKey"
EOF

# --- GitHub Actions workflow ---
cat > "$TP_DIR/github-actions.yml" << 'EOF'
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    env:
      AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
      AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
      # Hardcoded test secret (bad practice, but common)
      DEPLOY_TOKEN: "ghp_1234567890abcdefghij1234567890ABCDEF"
      NPM_TOKEN: "npm_1234567890abcdefghijklmnopqrstuvwx"
    steps:
      - uses: actions/checkout@v4
      - name: Deploy
        run: echo "Deploying with token $DEPLOY_TOKEN"
EOF

# --- Gradle properties ---
cat > "$TP_DIR/gradle-properties.gradle" << 'EOF'
// Gradle build configuration with secrets
// WARNING: Test secrets only

ext {
    awsAccessKey = 'AKIAIOSFODNN7EXAMPLE6'
    awsSecretKey = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY6'
    pypiApiToken = 'pypi-AgEIcHlwaS5vcmcAAABC1234567890abcdefghijklmnop'
    rubyGemsToken = 'rubygems_1234567890abcdefghijklmnopqrstuvwx'
}
EOF

# --- .npmrc ---
cat > "$TP_DIR/npm-rc.npmrc" << 'EOF'
# NPM configuration with auth token
registry=https://registry.npmjs.org/
//registry.npmjs.org/:_authToken=npm_1234567890abcdefghijklmnopqrstuvwx
always-auth=true
EOF

# --- PHP config ---
cat > "$TP_DIR/php-config.php" << 'EOF'
<?php
// PHP application configuration
// WARNING: Test secrets only

return [
    'database' => [
        'host' => 'db.example.com',
        'username' => 'admin',
        'password' => 'PhpDatabasePassword123!',
    ],
    
    'aws' => [
        'key' => 'AKIAIOSFODNN7EXAMPLE7',
        'secret' => 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY7',
    ],
    
    'stripe' => [
        'secret' => 'sk_live_1234567890abcdefghijklmnopqrstuv',
    ],
];
EOF

# --- Ruby initializer ---
cat > "$TP_DIR/ruby-initializer.rb" << 'EOF'
# Rails initializer with API keys
# WARNING: Test secrets only

Rails.application.configure do
  # AWS
  config.aws_access_key_id = 'AKIAIOSFODNN7EXAMPLE8'
  config.aws_secret_access_key = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY8'
  
  # Stripe
  config.stripe_api_key = 'sk_live_1234567890abcdefghijklmnopqrstuv'
  
  # SendGrid
  config.sendgrid_api_key = 'SG.abcdefghijklmnopqrstuvwx.1234567890abcdefghijklmnopqrstuvwxyz12345'
end
EOF

# --- Shell script ---
cat > "$TP_DIR/shell-script.sh" << 'EOF'
#!/bin/bash
# Deployment script with exported credentials
# WARNING: Test secrets only

export AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE9"
export AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY9"
export GITHUB_TOKEN="ghp_1234567890abcdefghij1234567890ABCDEF"
export DATABASE_URL="postgresql://admin:ShellScriptPassword123@db.example.com:5432/prod"

echo "Deploying with AWS credentials..."
EOF

# --- JSON config ---
cat > "$TP_DIR/json-config.json" << 'EOF'
{
  "application": {
    "name": "MyApp",
    "version": "1.0.0"
  },
  "credentials": {
    "aws": {
      "accessKeyId": "AKIAIOSFODNN7EXAMPLE10",
      "secretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY10"
    },
    "github": {
      "token": "ghp_1234567890abcdefghij1234567890ABCDEF"
    },
    "datadog": {
      "apiKey": "1234567890abcdef1234567890abcdef",
      "appKey": "abcdef1234567890abcdef1234567890abcdef12"
    }
  }
}
EOF

# --- XML config (Spring) ---
cat > "$TP_DIR/xml-config.xml" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<beans xmlns="http://www.springframework.org/schema/beans">
    
    <bean id="awsConfig" class="com.example.AwsConfig">
        <property name="accessKey" value="AKIAIOSFODNN7EXAMPLE11"/>
        <property name="secretKey" value="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY11"/>
    </bean>
    
    <bean id="databaseConfig" class="com.example.DatabaseConfig">
        <property name="password" value="SpringXmlPassword123!"/>
    </bean>
    
</beans>
EOF

# --- TOML config ---
cat > "$TP_DIR/toml-config.toml" << 'EOF'
# TOML configuration file
# WARNING: Test secrets only

[aws]
access_key_id = "AKIAIOSFODNN7EXAMPLE12"
secret_access_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY12"

[github]
token = "ghp_1234567890abcdefghij1234567890ABCDEF"

[database]
password = "TomlConfigPassword123!"

[stripe]
secret_key = "sk_live_1234567890abcdefghijklmnopqrstuv"
EOF

# --- CSV dump ---
cat > "$TP_DIR/csv-dump.csv" << 'EOF'
user_id,email,api_key,created_at
1,user1@example.com,AKIAIOSFODNN7EXAMPLE13,2024-01-01
2,user2@example.com,ghp_1234567890abcdefghij1234567890ABCDEF,2024-01-02
3,user3@example.com,sk_live_1234567890abcdefghijklmnopqrstuv,2024-01-03
4,admin@example.com,wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY13,2024-01-04
EOF

# --- SQL migration ---
cat > "$TP_DIR/sql-migration.sql" << 'EOF'
-- Database migration with hardcoded credentials
-- WARNING: Test secrets only

INSERT INTO api_keys (user_id, key_value, created_at)
VALUES 
    (1, 'AKIAIOSFODNN7EXAMPLE14', NOW()),
    (2, 'ghp_1234567890abcdefghij1234567890ABCDEF', NOW()),
    (3, 'sk_live_1234567890abcdefghijklmnopqrstuv', NOW());

-- Update admin password
UPDATE users SET password_hash = 'SqlMigrationPassword123!' WHERE username = 'admin';
EOF

# --- Jupyter notebook ---
cat > "$TP_DIR/jupyter-notebook.ipynb" << 'EOF'
{
 "cells": [
  {
   "cell_type": "code",
   "execution_count": 1,
   "metadata": {},
   "outputs": [],
   "source": [
    "# API credentials for data access\n",
    "AWS_ACCESS_KEY = 'AKIAIOSFODNN7EXAMPLE15'\n",
    "AWS_SECRET_KEY = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY15'\n",
    "OPENAI_API_KEY = 'sk-1234567890abcdefghijklmnopqrstuvwx1234567890ABCDEFGH'"
   ]
  }
 ],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 5
}
EOF

# --- Markdown documentation ---
cat > "$TP_DIR/markdown-doc.md" << 'EOF'
# API Integration Guide

## Setup

To use our API, you'll need to configure your credentials:

```bash
# Example configuration
export AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE16"
export AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY16"
export GITHUB_TOKEN="ghp_1234567890abcdefghij1234567890ABCDEF"
```

## Testing

For testing, use these sample credentials:
- Stripe Key: `sk_live_1234567890abcdefghijklmnopqrstuv`
- SendGrid Key: `SG.abcdefghijklmnopqrstuvwx.1234567890abcdefghijklmnopqrstuvwxyz12345`
EOF

echo "Created $(ls -1 "$TP_DIR" | wc -l) true-positive files in $TP_DIR"

# ============================================================================
# TRUE NEGATIVES (30+ files)
# ============================================================================

# Copy original 11 TN files
cp coax-benchmarks/datasets/secrets/true-negatives/*.txt "$TN_DIR" 2>/dev/null || true

# --- Placeholder config ---
cat > "$TN_DIR/placeholder-config.py" << 'EOF'
# Configuration file with placeholder values
# Replace these with your actual credentials

AWS_ACCESS_KEY_ID = "YOUR_AWS_ACCESS_KEY_HERE"
AWS_SECRET_ACCESS_KEY = "your-secret-key-here"
GITHUB_TOKEN = "<your-github-token>"
STRIPE_KEY = "CHANGE_ME"
DATABASE_PASSWORD = "TODO: Set password"
API_KEY = "INSERT_API_KEY_HERE"
SECRET = "REPLACE_WITH_YOUR_SECRET"
TOKEN = "your-token-here"
EOF

# --- Hash values ---
cat > "$TN_DIR/hash-values.json" << 'EOF'
{
  "files": {
    "README.md": "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "app.js": "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
    "config.yml": "sha512:cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
    "data.bin": "md5:d41d8cd98f00b204e9800998ecf8427e"
  },
  "integrity": {
    "package": "sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC"
  }
}
EOF

# --- UUID config ---
cat > "$TN_DIR/uuid-config.yaml" << 'EOF'
# Configuration with UUIDs (not secrets)
application:
  id: 550e8400-e29b-41d4-a716-446655440000
  session_id: 6ba7b810-9dad-11d1-80b4-00c04fd430c8
  request_id: 6ba7b812-9dad-11d1-80b4-00c04fd430c8
  trace_id: f47ac10b-58cc-4372-a567-0e02b2c3d479
  correlation_id: 123e4567-e89b-12d3-a456-426614174000
  deployment_id: 9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d
EOF

# --- Base64 content ---
cat > "$TN_DIR/base64-content.txt" << 'EOF'
# Base64 encoded content (not secrets)
# These are encoded images/data, not credentials

Image data:
iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==

Binary data:
SGVsbG8gV29ybGQhIFRoaXMgaXMgYmFzZTY0IGVuY29kZWQgdGV4dC4=

More encoded content:
VGhpcyBpcyBub3QgYSBzZWNyZXQsIGp1c3QgYmFzZTY0IGVuY29kZWQgZGF0YS4=
EOF

# --- Encrypted values ---
cat > "$TN_DIR/encrypted-values.env" << 'EOF'
# Encrypted vault values (not plaintext secrets)
# These are ENC[...] format from various vault systems

AWS_KEY=ENC[AES256_GCM,data:abc123,iv:def456,tag:ghi789,type:str]
DATABASE_PASSWORD=ENC[AES256_GCM,data:xyz789,iv:uvw123,tag:rst456,type:str]
API_TOKEN=ENC[AES256_GCM,data:mno345,iv:pqr678,tag:stu901,type:str]

# SOPS encrypted values
SECRET_KEY=ENC[AES256_GCM,data:longencryptedvalue1234567890abcdefghijklmnopqrstuvwxyz,iv:shortiv123,tag:tag123,type:str]
EOF

# --- Example docs ---
cat > "$TN_DIR/example-docs.md" << 'EOF'
# API Key Examples

This document shows example API key formats for documentation purposes.

## AWS Keys
Example format: `AKIAEXAMPLE12345678` (20 characters, starts with AKIA)

## GitHub Tokens  
Example format: `ghp_ExampleToken1234567890abcdef` (starts with ghp_)

## Stripe Keys
Example format: `sk_live_ExampleKey1234567890abcdefgh` (starts with sk_live_)

## Important
These are EXAMPLE formats only. Never use example keys in production.
Replace with your actual keys from the respective dashboards.
EOF

# --- URL parameters ---
cat > "$TN_DIR/url-parameters.log" << 'EOF'
# Web server logs with URL query parameters
# Long parameters are tracking IDs, not secrets

2024-01-01 10:00:00 GET /page?utm_source=google&utm_medium=cpc&utm_campaign=summer_sale&utm_content=ad1&utm_term=shoes&session_id=abc123def456ghi789jkl012mno345pqr678stu901vwx234yz
2024-01-01 10:01:00 GET /api/data?user_id=12345&tracking_id=xyz789abc123def456ghi789jkl012mno345pqr678stu901vwx234yz567&ref=homepage
2024-01-01 10:02:00 POST /submit?csrf_token=abc123&data=payload&analytics_id=longtrackingid1234567890abcdefghijklmnopqrstuvwxyz1234567890
EOF

# --- CSS hex colors ---
cat > "$TN_DIR/css-hex-colors.css" << 'EOF'
/* CSS file with hex colors (not secrets) */

:root {
    --primary-color: #3498db;
    --secondary-color: #2ecc71;
    --accent-color: #e74c3c;
    --text-color: #2c3e50;
    --background: #ecf0f1;
    --border: #bdc3c7;
    --shadow: rgba(0, 0, 0, 0.1);
}

.button {
    background-color: #007bff;
    border: 1px solid #0056b3;
    color: #ffffff;
}

.card {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

/* RGB and RGBA colors */
.alert {
    background-color: rgb(255, 0, 0);
    border-color: rgba(255, 0, 0, 0.5);
}
EOF

# --- JWT documentation ---
cat > "$TN_DIR/jwt-documentation.md" << 'EOF'
# JWT Token Format

A JWT token consists of three parts separated by dots:

```
header.payload.signature
```

## Example Format

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

## Parts Explained

1. **Header**: Contains token type and algorithm
   - Example: `{"alg": "HS256", "typ": "JWT"}`

2. **Payload**: Contains claims (user data)
   - Example: `{"sub": "1234567890", "name": "John Doe", "iat": 1516239022}`

3. **Signature**: Verifies token integrity
   - Created using header, payload, and secret

## Important

This is documentation only. The example token above is for illustration.
Never share real JWT tokens in documentation.
EOF

# --- Minified code ---
cat > "$TN_DIR/minified-code.min.js" << 'EOF'
!function(e,t){"use strict";"object"==typeof module&&"object"==typeof module.exports?module.exports=e.document?t(e,!0):function(e){if(!e.document)throw new Error("jQuery requires a window with a document");return t(e)}:t(e)}("undefined"!=typeof window?window:this,function(e,t){"use strict";var n=[],r=e.document,i=Object.getPrototypeOf,o=n.slice,a=n.concat,s=n.push,u=n.indexOf,l={},c=l.toString,f=l.hasOwnProperty,p=f.toString,d=p.call(Object),h={},g=function e(t){return"function"==typeof t&&"number"!=typeof t.nodeType},v=function e(t){return null!=t&&t===t.window},m={type:!0,src:!0,nonce:!0,noModule:!0};function y(e,t,n){var i,o,a=(n=n||r).createElement("script");if(a.text=e,t)for(i in m)(o=t[i]||t.getAttribute&&t.getAttribute(i))&&a.setAttribute(i,o);n.head.appendChild(a).parentNode.removeChild(a)}function b(e){return null==e?e+"":"object"==typeof e||"function"==typeof e?l[c.call(e)]||"object":typeof e}var w="3.6.0",T=function(e,t){return new T.fn.init(e,t)};T.fn=T.prototype={jquery:w,constructor:T,length:0,toArray:function(){return o.call(this)},get:function(e){return null==e?o.call(this):e<0?this[e+this.length]:this[e]},pushStack:function(e){var t=T.merge(this.constructor(),e);return t.prevObject=this,t},each:function(e){return T.each(this,e)},map:function(e){return this.pushStack(T.map(this,function(t,n){return e.call(t,n,t)}))},slice:function(){return this.pushStack(o.apply(this,arguments))},first:function(){return this.eq(0)},last:function(){return this.eq(-1)},even:function(){return this.pushStack(T.grep(this,function(e,t){return(t+1)%2}))},odd:function(){return this.pushStack(T.grep(this,function(e,t){return t%2}))},eq:function(e){var t=this.length,n=+e+(e<0?t:0);return this.pushStack(n>=0&&n<t?[this[n]]:[])},end:function(){return this.prevObject||this.constructor()},push:s,sort:n.sort,splice:n.splice},T.extend=T.fn.extend=function(){var e,t,n,r,i,o,a=arguments[0]||{},s=1,u=arguments.length,l=!1;for("boolean"==typeof a&&(l=a,a=arguments[s]||{},s++),"object"==typeof a||g(a)||(a={}),s===u&&(a=this,s--);s<u;s++)if(null!=(e=arguments[s]))for(t in e)r=e[t],"__proto__"!==t&&a!==r&&(l&&r&&(T.isPlainObject(r)||(i=Array.isArray(r)))?(i?(i=!1,o=a[t]||[]):o=a[t]||{}):o=a[t],a[t]=T.extend(l,o,r)):void 0!==r&&(a[t]=r);return a},T.extend({expando:"jQuery"+(w+Math.random()).replace(/\D/g,""),isReady:!0,error:function(e){throw new Error(e)},noop:function(){},isPlainObject:function(e){var t,n;return!(!e||"[object Object]"!==c.call(e))&&(!(t=i(e))||"function"==typeof(n=f.call(t,"constructor")&&t.constructor)&&p.call(n)===d)},isEmptyObject:function(e){var t;for(t in e)return!1;return!0},globalEval:function(e,t,n){y(e,{nonce:t&&t.nonce},n)},each:function(e,t){var n,r=0;if(C(e)){for(n=e.length;r<n;r++)if(!1===t.call(e[r],r,e[r]))break}else for(r in e)if(!1===t.call(e[r],r,e[r]))break;return e},makeArray:function(e,t){var n=t||[];return null!=e&&(C(Object(e))?T.merge(n,"string"==typeof e?[e]:e):s.call(n,e)),n},inArray:function(e,t,n){return null==t?-1:u.call(t,e,n)},merge:function(e,t){for(var n=+t.length,r=0,i=e.length;r<n;r++)e[i++]=t[r];return e.length=i,e},grep:function(e,t,n){for(var r=[],i=0,o=e.length,a=!n;i<o;i++)!t(e[i],i)!==a&&r.push(e[i]);return r},map:function(e,t,n){var r,i,o=0,s=[];if(C(e))for(r=e.length;o<r;o++)null!=(i=t(e[o],o,n))&&s.push(i);else for(o in e)null!=(i=t(e[o],o,n))&&s.push(i);return a.apply([],s)},guid:1,support:h}),"function"==typeof Symbol&&(T.fn[Symbol.iterator]=n[Symbol.iterator]),T.each("Boolean Number String Function Array Date RegExp Object Error Symbol".split(" "),function(e,t){l["[object "+t+"]"]=t.toLowerCase()});function C(e){var t=!!e&&"length"in e&&e.length,n=b(e);return!g(e)&&!v(e)&&("array"===n||0===t||"number"==typeof t&&t>0&&t-1 in e)}var E=function(e){var t,n,r,i,o,a,s,u,l,c,f,p,d,h,g,v,m,y,b,w="sizzle"+1*new Date,T=e.document,E=0,S=0,C=ue(),k=ue(),A=ue(),N=ue(),j=function(e,t){return e===t&&(f=!0),0},D={}.hasOwnProperty,L=[],q=L.pop,M=L.push,O=L.push,P=L.slice,H=function(e,t){for(var n=0,r=e.length;n<r;n++)if(e[n]===t)return n;return-1},R="checked|selected|async|autofocus|autoplay|controls|defer|disabled|hidden|ismap|loop|multiple|open|readonly|required|scoped",I="[\\x20\\t\\r\\n\\f]",W="(?:\\\\[\\da-fA-F]{1,6}"+I+"?|\\\\[^\\r\\n\\f]|[\\w-]|[^\0-\\x7f])+",B="\\["+I+"*("+W+")(?:"+I+"*([*^$|!~]?=)"+I+"*(?:'((?:\\\\.|[^\\\\'])*)'|\"((?:\\\\.|[^\\\\\"])*)\"|("+W+"))|)"+I+"*\\]",F=":("+W+")(?:\\((('((?:\\\\.|[^\\\\'])*)'|\"((?:\\\\.|[^\\\\\"])*)\")|((?:\\\\.|[^\\\\()[\\]]|"+B+")*)|.*)\\)|)",$=new RegExp(I+"+","g"),_=new RegExp("^"+I+"+|((?:^|[^\\\\])(?:\\\\.)*)"+I+"+$","g"),z=new RegExp("^"+I+"*,"+I+"*"),X=new RegExp("^"+I+"*([>+~]|"+I+")"+I+"*"),U=new RegExp(I+"|>"),V=new RegExp(F),G=new RegExp("^"+W+"$"),Y={ID:new RegExp("^#("+W+")"),CLASS:new RegExp("^\\.("+W+")"),TAG:new RegExp("^("+W+"|[*])"),ATTR:new RegExp("^"+B),PSEUDO:new RegExp("^"+F),CHILD:new RegExp("^:(only|first|last|nth|nth-last)-(child|of-type)(?:\\("+I+"*(even|odd|(([+-]|)(\\d*)n|)"+I+"*(?:([+-]|)"+I+"*(\\d+)|))"+I+"*\\)|)","i"),bool:new RegExp("^(?:"+R+")$","i"),needsContext:new RegExp("^"+I+"*[>+~]|:(even|odd|eq|gt|lt|nth|first|last)(?:\\("+I+"*((?:-\\d)?\\d*)"+I+"*\\)|)(?=[^-]|$)","i")},Q=/HTML$/i,J=/^(?:input|select|textarea|button)$/i,K=/^h\d$/i,Z=/^[^{]+\{\s*\[native \w/,ee=/^(?:#([\w-]+)|(\w+)|\.([\w-]+))$/,te=/[+~]/,ne=new RegExp("\\\\[\\da-fA-F]{1,6}"+I+"?|\\\\([^\\r\\n\\f])","g"),re=function(e,t){var n="0x"+e.slice(1)-65536;return t||(n<0?String.fromCharCode(n+65536):String.fromCharCode(n>>10|55296,1023&n|56320))},ie=/([\0-\x1f\x7f]|^-?\d)|^-$|[^\0-\x1f\x7f-\uFFFF\w-]/g,oe=function(e,t){return t?"\0"===e?"\ufffd":e.slice(0,-1)+"\\"+e.charCodeAt(e.length-1).toString(16)+" ":"\\"+e},ae=function(){p()},se=ve(function(e){return!0===e.disabled&&"fieldset"===e.nodeName.toLowerCase()},{dir:"parentNode",next:"legend"});try{O.apply(L=P.call(T.childNodes),T.childNodes),L[T.childNodes.length].nodeType}catch(e){O={apply:L.length?function(e,t){M.apply(e,P.call(t))}:function(e,t){var n=e.length,r=0;while(e[n++]=t[r++]);e.length=n-1}}}function ue(e,t,n,r){var i,o,s,u,l,c,f,d=t&&t.ownerDocument,h=t?t.nodeType:9;if(n=n||[],"string"!=typeof e||!e||1!==h&&9!==h&&11!==h)return n;if(!r&&(p(t),t=t||d,g)){if(11!==h&&(l=ee.exec(e)))if(i=l[1]){if(9===h){if(!(s=t.getElementById(i)))return n;if(s.id===i)return n.push(s),n}else if(d&&(s=d.getElementById(i))&&y(t,s)&&s.id===i)return n.push(s),n}else{if(l[2])return O.apply(n,t.getElementsByTagName(e)),n;if((i=l[3])&&t.getElementsByClassName)return O.apply(n,t.getElementsByClassName(i)),n}if(!(N[e+" "]||m&&m.test(e))){if(u=e,f=t,1===h&&(U.test(e)||X.test(e))){(d=te.test(e)&&ye(t.parentNode)||t)===t&&h.scope||((s=t.getAttribute("id"))?s=s.replace(ie,oe):t.setAttribute("id",s=w)),o=(c=a(e)).length;while(o--)c[o]=(s?"#"+s:":scope")+" "+xe(c[o]);u=c.join(",")}try{return O.apply(n,d.querySelectorAll(u)),n}catch(t){N(e,!0)}finally{s===w&&t.removeAttribute("id")}}}return b(e.replace(_,"$1"),t,n,r)}function le(){var e=[];return function t(n,i){return e.push(n+" ")>r.cacheLength&&delete t[e.shift()],t[n+" "]=i}}function ce(e){return e[w]=!0,e}function fe(e){var t=d.createElement("fieldset");try{return!!e(t)}catch(e){return!1}finally{t.parentNode&&t.parentNode.removeChild(t),t=null}}function pe(e,t){var n=e.split("|"),i=r.createDocumentFragment();if(i.createElement)while(n.length)i.createElement(n.pop());return i}!function(){var e=d.createElement("script");e.type="module",e.supportsModule=!0,T.head.appendChild(e),h.moduleSupport=!!e.supportsModule,T.head.removeChild(e)}();var de,he,ge,ve,me,ye,be,we,Te,Ee,Se,Ce,ke,Ae,Ne,je,De,Le,qe,Me,Oe,Pe,He,Re,Ie,We,Be,Fe,$e,ze,Xe,Ue,Ve,Ge,Ye,Qe,Je,Ke,Ze,et,tt,nt,rt,it,ot,at,st,ut,lt,ct,ft,pt,dt,ht,gt,vt,mt,yt,bt,wt,Tt,Et,St,Ct,kt,At,Nt,jt,Dt,Lt,qt,Mt,Ot,Pt,Ht,Rt,It,Wt,Bt,Ft,$t,zt,Xt,Ut,Vt,Gt,Yt,Qt,Jt,Kt,Zt,en,tn,nn,rn,on,an,sn,un,ln,cn,fn,pn,dn,hn,gn,vn,mn,yn,bn,wn,Tn,En,Sn,Cn,kn,An,Nn,jn,Dn,Ln,qn,Mn,On,Pn,Hn,Rn,In,Wn,Bn,Fn,$n,zn,Xn,Un,Vn,Gn,Yn,Qn,Jn,Kn,Zn,er,tr,nr,rr,ir,or,ar,sr,ur,lr,cr,fr,pr,dr,hr,gr,vr,mr,yr,br,wr,Tr,Er,Sr,Cr,kr,Ar,Nr,jr,Dr,Lr,qr,Mr,Or,Pr,Hr,Rr,Ir,Wr,Br,Fr,$r,zr,Xr,Ur,Vr,Gr,Yr,Qr,Jr,Kr,Zr,e2,t2,n2,r2,i2,o2,a2,s2,u2,l2,c2,f2,p2,d2,h2,g2,v2,m2,y2,b2,w2,T2,E2,S2,C2,k2,A2,N2,j2,D2,L2,q2,M2,O2,P2,H2,R2,I2,W2,B2,F2,$2,z2,X2,U2,V2,G2,Y2,Q2,J2,K2,Z2,e3,t3,n3,r3,i3,o3,a3,s3,u3,l3,c3,f3,p3,d3,h3,g3,v3,m3,y3,b3,w3,T3,E3,S3,C3,k3,A3,N3,j3,D3,L3,q3,M3,O3,P3,H3,R3,I3,W3,B3,F3,$3,z3,X3,U3,V3,G3,Y3,Q3,J3,K3,Z3,e4,t4,n4,r4,i4,o4,a4,s4,u4,l4,c4,f4,p4,d4,h4,g4,v4,m4,y4,b4,w4,T4,E4,S4,C4,k4,A4,N4,j4,D4,L4,q4,M4,O4,P4,H4,R4,I4,W4,B4,F4,$4,z4,X4,U4,V4,G4,Y4,Q4,J4,K4,Z4,e5,t5,n5,r5,i5,o5,a5,s5,u5,l5,c5,f5,p5,d5,h5,g5,v5,m5,y5,b5,w5,T5,E5,S5,C5,k5,A5,N5,j5,D5,L5,q5,M5,O5,P5,H5,R5,I5,W5,B5,F5,$5,z5,X5,U5,V5,G5,Y5,Q5,J5,K5,Z5,e6,t6,n6,r6,i6,o6,a6,s6,u6,l6,c6,f6,p6,d6,h6,g6,v6,m6,y6,b6,w6,T6,E6,S6,C6,k6,A6,N6,j6,D6,L6,q6,M6,O6,P6,H6,R6,I6,W6,B6,F6,$6,z6,X6,U6,V6,G6,Y6,Q6,J6,K6,Z6,e7,t7,n7,r7,i7,o7,a7,s7,u7,l7,c7,f7,p7,d7,h7,g7,v7,m7,y7,b7,w7,T7,E7,S7,C7,k7,A7,N7,j7,D7,L7,q7,M7,O7,P7,H7,R7,I7,W7,B7,F7,$7,z7,X7,U7,V7,G7,Y7,Q7,J7,K7,Z7,e8,t8,n8,r8,i8,o8,a8,s8,u8,l8,c8,f8,p8,d8,h8,g8,v8,m8,y8,b8,w8,T8,E8,S8,C8,k8,A8,N8,j8,D8,L8,q8,M8,O8,P8,H8,R8,I8,W8,B8,F8,$8,z8,X8,U8,V8,G8,Y8,Q8,J8,K8,Z8,e9,t9,n9,r9,i9,o9,a9,s9,u9,l9,c9,f9,p9,d9,h9,g9,v9,m9,y9,b9,w9,T9,E9,S9,C9,k9,A9,N9,j9,D9,L9,q9,M9,O9,P9,H9,R9,I9,W9,B9,F9,$9,z9,X9,U9,V9,G9,Y9,Q9,J9,K9,Z9,ea,ta,na,ra,ia,oa,aa,sa,ua,la,ca,fa,pa,da,ha,ga,va,ma,ya,ba,wa,Ta,Ea,Sa,Ca,ka,Aa,Na,ja,Da,La,qa,Ma,Oa,Pa,Ha,Ra,Ia,Wa,Ba,Fa,$a,za,Xa,Ua,Va,Ga,Ya,Qa,Ja,Ka,Za,eb,tb,nb,rb,ib,ob,ab,sb,ub,lb,cb,fb,pb,db,hb,gb,vb,mb,yb,bb,wb,Tb,Eb,Sb,Cb,kb,Ab,Nb,jb,Db,Lb,qb,Mb,Ob,Pb,Hb,Rb,Ib,Wb,Bb,Fb,$b,zb,Xb,Ub,Vb,Gb,Yb,Qb,Jb,Kb,Zb,et2,tt2,nt2,rt2,it2,ot2,at2,st2,ut2,lt2,ct2,ft2,pt2,dt2,ht2,gt2,vt2,mt2,yt2,bt2,wt2,Tt2,Et2,St2,Ct2,kt2,At2,Nt2,jt2,Dt2,Lt2,qt2,Mt2,Ot2,Pt2,Ht2,Rt2,It2,Wt2,Bt2,Ft2,$t2,zt2,Xt2,Ut2,Vt2,Gt2,Yt2,Qt2,Jt2,Kt2,Zt2,en2,tn2,nn2,rn2,on2,an2,sn2,un2,ln2,cn2,fn2,pn2,dn2,hn2,gn2,vn2,mn2,yn2,bn2,wn2,Tn2,En2,Sn2,Cn2,kn2,An2,Nn2,jn2,Dn2,Ln2,qn2,Mn2,On2,Pn2,Hn2,Rn2,In2,Wn2,Bn2,Fn2,$n2,zn2,Xn2,Un2,Vn2,Gn2,Yn2,Qn2,Jn2,Kn2,Zn2,e10,t10,n10,r10,i10,o10,a10,s10,u10,l10,c10,f10,p10,d10,h10,g10,v10,m10,y10,b10,w10,T10,E10,S10,C10,k10,A10,N10,j10,D10,L10,q10,M10,O10,P10,H10,R10,I10,W10,B10,F10,$10,z10,X10,U10,V10,G10,Y10,Q10,J10,K10,Z10;return function(e){var t,n=[],r=0,i=e.length;for(;r<i;r++)(t=e[r])&&(n[r]=t);return n}(arguments)}).call(this);
EOF

# --- Random test data ---
cat > "$TN_DIR/random-test-data.py" << 'EOF'
# Test data generation script
# Generates random strings for testing (not secrets)

import random
import string

def generate_random_string(length=32):
    """Generate a random alphanumeric string."""
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

def generate_test_data():
    """Generate test data for unit tests."""
    return {
        'random_id': generate_random_string(36),
        'session_token': generate_random_string(64),
        'api_response_hash': generate_random_string(128),
        'checksum': generate_random_string(64),
    }

# Example output (not real secrets):
# random_id: "aB3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5z"
# session_token: "aB3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5zA7bC9dE1fG3hI5jK7lM9nO1pQ3rS5tU7vW9xY1zA3bC5dE7fG9hI1jK3lM5nO7pQ9rS1tU3vW5xY7zA9bC1dE3fG5hI7jK9lM1nO3pQ5rS7tU9vW1xY3zA5bC7dE9fG1hI3jK5lM7nO9pQ1rS3tU5vW7xY9zA1bC3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5z"
# api_response_hash: "aB3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5zA7bC9dE1fG3hI5jK7lM9nO1pQ3rS5tU7vW9xY1zA3bC5dE7fG9hI1jK3lM5nO7pQ9rS1tU3vW5xY7zA9bC1dE3fG5hI7jK9lM1nO3pQ5rS7tU9vW1xY3zA5bC7dE9fG1hI3jK5lM7nO9pQ1rS3tU5vW7xY9zA1bC3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5zA7bC9dE1fG3hI5jK7lM9nO1pQ3rS5tU7vW9xY1zA3bC5dE7fG9hI1jK3lM5nO7pQ9rS1tU3vW5xY7zA9bC1dE3fG5hI7jK9lM1nO3pQ5rS7tU9vW1xY3zA5bC7dE9fG1hI3jK5lM7nO9pQ1rS3tU5vW7xY9zA1bC3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5z"
# checksum: "aB3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5zA7bC9dE1fG3hI5jK7lM9nO1pQ3rS5tU7vW9xY1zA3bC5dE7fG9hI1jK3lM5nO7pQ9rS1tU3vW5xY7zA9bC1dE3fG5hI7jK9lM1nO3pQ5rS7tU9vW1xY3zA5bC7dE9fG1hI3jK5lM7nO9pQ1rS3tU5vW7xY9zA1bC3dE5fG7hI9jK1lM3nO5pQ7rS9tU1vW3xY5z"
EOF

echo "Created $(ls -1 "$TN_DIR" | wc -l) true-negative files in $TN_DIR"

echo ""
echo "Expanded benchmark corpus created successfully!"
echo "True Positives: $(ls -1 "$TP_DIR" | wc -l) files"
echo "True Negatives: $(ls -1 "$TN_DIR" | wc -l) files"

# ============================================================================
# ADDITIONAL TRUE POSITIVES (to reach 50+)
# ============================================================================

# --- Go config ---
cat > "$TP_DIR/go-config.go" << 'EOF'
package main

// Configuration with API keys
// WARNING: Test secrets only

var (
    AWSAccessKey = "AKIAIOSFODNN7EXAMPLE20"
    AWSSecretKey = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY20"
    GitHubToken = "ghp_1234567890abcdefghij1234567890ABCDEF"
    StripeKey = "sk_live_1234567890abcdefghijklmnopqrstuv"
)
EOF

# --- Rust config ---
cat > "$TP_DIR/rust-config.rs" << 'EOF'
// Rust application configuration
// WARNING: Test secrets only

const AWS_ACCESS_KEY: &str = "AKIAIOSFODNN7EXAMPLE21";
const AWS_SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY21";
const GITHUB_TOKEN: &str = "ghp_1234567890abcdefghij1234567890ABCDEF";
const DATABASE_URL: &str = "postgresql://admin:RustDbPassword123@localhost:5432/mydb";
EOF

# --- Java properties ---
cat > "$TP_DIR/java-properties.properties" << 'EOF'
# Java application properties
# WARNING: Test secrets only

aws.accessKeyId=AKIAIOSFODNN7EXAMPLE22
aws.secretKey=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY22
github.token=ghp_1234567890abcdefghij1234567890ABCDEF
stripe.secret.key=sk_live_1234567890abcdefghijklmnopqrstuv
database.password=JavaPropertiesPassword123!
EOF

# --- C# config ---
cat > "$TP_DIR/csharp-config.cs" << 'EOF'
// C# application configuration
// WARNING: Test secrets only

public static class Config {
    public const string AwsAccessKey = "AKIAIOSFODNN7EXAMPLE23";
    public const string AwsSecretKey = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY23";
    public const string GitHubToken = "ghp_1234567890abcdefghij1234567890ABCDEF";
    public const string ConnectionString = "Server=localhost;Database=mydb;User Id=admin;Password=CSharpPassword123!";
}
EOF

# --- Scala config ---
cat > "$TP_DIR/scala-config.scala" << 'EOF'
// Scala application configuration
// WARNING: Test secrets only

object Config {
  val awsAccessKey = "AKIAIOSFODNN7EXAMPLE24"
  val awsSecretKey = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY24"
  val githubToken = "ghp_1234567890abcdefghij1234567890ABCDEF"
  val databasePassword = "ScalaPassword123!"
}
EOF

# --- Kotlin config ---
cat > "$TP_DIR/kotlin-config.kt" << 'EOF'
// Kotlin application configuration
// WARNING: Test secrets only

object Config {
    const val AWS_ACCESS_KEY = "AKIAIOSFODNN7EXAMPLE25"
    const val AWS_SECRET_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY25"
    const val GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890ABCDEF"
    const val DB_PASSWORD = "KotlinPassword123!"
}
EOF

# --- Elixir config ---
cat > "$TP_DIR/elixir-config.exs" << 'EOF'
# Elixir configuration
# WARNING: Test secrets only

use Mix.Config

config :myapp,
  aws_access_key: "AKIAIOSFODNN7EXAMPLE26",
  aws_secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY26",
  github_token: "ghp_1234567890abcdefghij1234567890ABCDEF",
  database_password: "ElixirPassword123!"
EOF

# --- Erlang config ---
cat > "$TP_DIR/erlang-config.config" << 'EOF'
% Erlang configuration
% WARNING: Test secrets only

[
  {myapp, [
    {aws_access_key, "AKIAIOSFODNN7EXAMPLE27"},
    {aws_secret_key, "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY27"},
    {github_token, "ghp_1234567890abcdefghij1234567890ABCDEF"},
    {database_password, "ErlangPassword123!"}
  ]}
].
EOF

# --- Haskell config ---
cat > "$TP_DIR/haskell-config.yaml" << 'EOF'
# Haskell application configuration
# WARNING: Test secrets only

aws:
  accessKey: "AKIAIOSFODNN7EXAMPLE28"
  secretKey: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY28"

github:
  token: "ghp_1234567890abcdefghij1234567890ABCDEF"

database:
  password: "HaskellPassword123!"
EOF

# --- Perl script ---
cat > "$TP_DIR/perl-script.pl" << 'EOF'
#!/usr/bin/perl
# Perl script with credentials
# WARNING: Test secrets only

use strict;
use warnings;

my $aws_access_key = 'AKIAIOSFODNN7EXAMPLE29';
my $aws_secret_key = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY29';
my $github_token = 'ghp_1234567890abcdefghij1234567890ABCDEF';
my $db_password = 'PerlPassword123!';

print "Configured with AWS key: $aws_access_key\n";
EOF

# --- Lua config ---
cat > "$TP_DIR/lua-config.lua" << 'EOF'
-- Lua configuration
-- WARNING: Test secrets only

local config = {
    aws = {
        access_key = "AKIAIOSFODNN7EXAMPLE30",
        secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY30"
    },
    github = {
        token = "ghp_1234567890abcdefghij1234567890ABCDEF"
    },
    database = {
        password = "LuaPassword123!"
    }
}

return config
EOF

# --- R script ---
cat > "$TP_DIR/r-script.R" << 'EOF'
# R script with API credentials
# WARNING: Test secrets only

# AWS credentials
aws_access_key <- "AKIAIOSFODNN7EXAMPLE31"
aws_secret_key <- "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY31"

# GitHub token
github_token <- "ghp_1234567890abcdefghij1234567890ABCDEF"

# Database connection
db_password <- "RPassword123!"

# API calls
library(httr)
GET("https://api.example.com", 
    add_headers(Authorization = paste("Bearer", github_token)))
EOF

# --- MATLAB script ---
cat > "$TP_DIR/matlab-script.m" << 'EOF'
% MATLAB script with credentials
% WARNING: Test secrets only

% AWS credentials
aws_access_key = 'AKIAIOSFODNN7EXAMPLE32';
aws_secret_key = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY32';

% GitHub token
github_token = 'ghp_1234567890abcdefghij1234567890ABCDEF';

% Database password
db_password = 'MatlabPassword123!';

% Use credentials
% ...
EOF

# --- PowerShell script ---
cat > "$TP_DIR/powershell-script.ps1" << 'EOF'
# PowerShell script with credentials
# WARNING: Test secrets only

$awsAccessKey = "AKIAIOSFODNN7EXAMPLE33"
$awsSecretKey = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY33"
$githubToken = "ghp_1234567890abcdefghij1234567890ABCDEF"
$dbPassword = "PowerShellPassword123!"

# Use credentials
# ...
EOF

# --- Batch script ---
cat > "$TP_DIR/batch-script.bat" << 'EOF'
@echo off
REM Batch script with credentials
REM WARNING: Test secrets only

set AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE34
set AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY34
set GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890ABCDEF
set DB_PASSWORD=BatchPassword123!

echo Configured with AWS key: %AWS_ACCESS_KEY_ID%
EOF

# --- Ansible playbook ---
cat > "$TP_DIR/ansible-playbook.yml" << 'EOF'
---
# Ansible playbook with secrets
# WARNING: Test secrets only

- hosts: all
  vars:
    aws_access_key: "AKIAIOSFODNN7EXAMPLE35"
    aws_secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY35"
    github_token: "ghp_1234567890abcdefghij1234567890ABCDEF"
    database_password: "AnsiblePassword123!"
  
  tasks:
    - name: Configure AWS
      debug:
        msg: "Configuring AWS with key {{ aws_access_key }}"
EOF

# --- Chef recipe ---
cat > "$TP_DIR/chef-recipe.rb" << 'EOF'
# Chef recipe with credentials
# WARNING: Test secrets only

node.default['aws']['access_key'] = 'AKIAIOSFODNN7EXAMPLE36'
node.default['aws']['secret_key'] = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY36'
node.default['github']['token'] = 'ghp_1234567890abcdefghij1234567890ABCDEF'
node.default['database']['password'] = 'ChefPassword123!'
EOF

# --- Puppet manifest ---
cat > "$TP_DIR/puppet-manifest.pp" << 'EOF'
# Puppet manifest with secrets
# WARNING: Test secrets only

$aws_access_key = 'AKIAIOSFODNN7EXAMPLE37'
$aws_secret_key = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY37'
$github_token = 'ghp_1234567890abcdefghij1234567890ABCDEF'
$database_password = 'PuppetPassword123!'

class myapp {
  # Use credentials
}
EOF

# --- Salt state ---
cat > "$TP_DIR/salt-state.sls" << 'EOF'
# Salt state with credentials
# WARNING: Test secrets only

myapp:
  config:
    - aws_access_key: AKIAIOSFODNN7EXAMPLE38
    - aws_secret_key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY38
    - github_token: ghp_1234567890abcdefghij1234567890ABCDEF
    - database_password: SaltPassword123!
EOF

# --- Vagrantfile ---
cat > "$TP_DIR/vagrantfile" << 'EOF'
# Vagrant configuration with secrets
# WARNING: Test secrets only

Vagrant.configure("2") do |config|
  config.vm.define "app" do |app|
    app.vm.provision "shell", inline: <<-SHELL
      export AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE39"
      export AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY39"
      export GITHUB_TOKEN="ghp_1234567890abcdefghij1234567890ABCDEF"
      export DB_PASSWORD="VagrantPassword123!"
    SHELL
  end
end
EOF

# --- Makefile ---
cat > "$TP_DIR/Makefile" << 'EOF'
# Makefile with credentials
# WARNING: Test secrets only

AWS_ACCESS_KEY_ID = AKIAIOSFODNN7EXAMPLE40
AWS_SECRET_ACCESS_KEY = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY40
GITHUB_TOKEN = ghp_1234567890abcdefghij1234567890ABCDEF
DATABASE_PASSWORD = MakefilePassword123!

deploy:
	@echo "Deploying with AWS key: $(AWS_ACCESS_KEY_ID)"
	@aws s3 sync ./dist s3://my-bucket/
EOF

# --- CI/CD config (GitLab CI) ---
cat > "$TP_DIR/gitlab-ci.yml" << 'EOF'
# GitLab CI configuration with secrets
# WARNING: Test secrets only

variables:
  AWS_ACCESS_KEY_ID: "AKIAIOSFODNN7EXAMPLE41"
  AWS_SECRET_ACCESS_KEY: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY41"
  GITHUB_TOKEN: "ghp_1234567890abcdefghij1234567890ABCDEF"
  DATABASE_PASSWORD: "GitLabCIPassword123!"

stages:
  - deploy

deploy:
  stage: deploy
  script:
    - echo "Deploying with AWS credentials"
    - aws s3 sync ./dist s3://my-bucket/
EOF

# --- Jenkinsfile ---
cat > "$TP_DIR/Jenkinsfile" << 'EOF'
// Jenkins pipeline with credentials
// WARNING: Test secrets only

pipeline {
    agent any
    
    environment {
        AWS_ACCESS_KEY_ID = 'AKIAIOSFODNN7EXAMPLE42'
        AWS_SECRET_ACCESS_KEY = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY42'
        GITHUB_TOKEN = 'ghp_1234567890abcdefghij1234567890ABCDEF'
        DATABASE_PASSWORD = 'JenkinsPassword123!'
    }
    
    stages {
        stage('Deploy') {
            steps {
                sh 'aws s3 sync ./dist s3://my-bucket/'
            }
        }
    }
}
EOF

# --- CircleCI config ---
cat > "$TP_DIR/circleci-config.yml" << 'EOF'
# CircleCI configuration with secrets
# WARNING: Test secrets only

version: 2.1

jobs:
  deploy:
    docker:
      - image: cimg/base:2023.01
    environment:
      AWS_ACCESS_KEY_ID: "AKIAIOSFODNN7EXAMPLE43"
      AWS_SECRET_ACCESS_KEY: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY43"
      GITHUB_TOKEN: "ghp_1234567890abcdefghij1234567890ABCDEF"
      DATABASE_PASSWORD: "CircleCIPassword123!"
    steps:
      - checkout
      - run: aws s3 sync ./dist s3://my-bucket/

workflows:
  deploy:
    jobs:
      - deploy
EOF

# --- Travis CI config ---
cat > "$TP_DIR/travis-ci.yml" << 'EOF'
# Travis CI configuration with secrets
# WARNING: Test secrets only

language: node_js
node_js:
  - "18"

env:
  global:
    - AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE44"
    - AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY44"
    - GITHUB_TOKEN="ghp_1234567890abcdefghij1234567890ABCDEF"
    - DATABASE_PASSWORD="TravisCIPassword123!"

script:
  - npm test

deploy:
  provider: s3
  access_key_id: $AWS_ACCESS_KEY_ID
  secret_access_key: $AWS_SECRET_ACCESS_KEY
  bucket: my-bucket
EOF

echo "Updated TP count: $(ls -1 "$TP_DIR" | wc -l) files"

# ============================================================================
# ADDITIONAL TRUE NEGATIVES (to reach 30+)
# ============================================================================

# --- Private SSH key (fake) ---
cat > "$TN_DIR/fake-ssh-key.pub" << 'EOF'
# This is a FAKE SSH public key for testing
# NOT a real key - just test data

ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC7fake1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789fake== test@example.com
EOF

# --- SSL certificate (fake) ---
cat > "$TN_DIR/fake-ssl-cert.pem" << 'EOF'
-----BEGIN CERTIFICATE-----
# This is a FAKE SSL certificate for testing
# NOT a real certificate - just test data

MIICfake1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRST
UVWXYZ0123456789+/fake1234567890abcdefghijklmnopqrstuvwxyzABCDEF
GHIJKLMNOPQRSTUVWXYZ0123456789+/fake1234567890abcdefghijklmnopqrst
uvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/fake==
-----END CERTIFICATE-----
EOF

# --- Binary file signature ---
cat > "$TN_DIR/binary-signature.bin" << 'EOF'
# Fake binary file signature
# NOT real binary data - just test patterns

PKfake1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUV
WXYZfake0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRS
TUVWXYZfake0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ
RSTUVWXYZfake0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNO
PQRSTUVWXYZfake0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLM
EOF

# --- Version strings ---
cat > "$TN_DIR/version-strings.txt" << 'EOF'
# Version strings and build IDs (not secrets)

Application version: 1.2.3-build.456+abcdef123456
Build ID: sha256-abc123def456ghi789jkl012mno345pqr678stu901vwx234yz
Commit hash: abc123def456ghi789jkl012mno345pqr678stu901
Release tag: v2.0.0-beta.1+build.789
EOF

# --- Feature flags ---
cat > "$TN_DIR/feature-flags.json" << 'EOF'
{
  "features": {
    "new_dashboard": true,
    "dark_mode": false,
    "beta_features": true,
    "analytics_v2": false,
    "experimental_api": true
  },
  "rollout_percentages": {
    "new_dashboard": 50,
    "dark_mode": 25,
    "beta_features": 100
  },
  "ab_test_ids": [
    "test_1234567890abcdef",
    "test_0987654321fedcba",
    "test_abcdef1234567890"
  ]
}
EOF

# --- Analytics IDs ---
cat > "$TN_DIR/analytics-ids.js" << 'EOF'
// Analytics and tracking IDs (not secrets)
// These are public identifiers

const ANALYTICS = {
    google: 'UA-123456789-1',
    google_gtag: 'G-ABCDEF1234',
    facebook: '1234567890123456',
    twitter: 'abcdefghijklmnop',
    linkedin: '1234567',
    hotjar: '1234567',
    fullstory: 'abc123def456',
    segment: 'abc123DEF456ghi789JKL012'
};

export default ANALYTICS;
EOF

# --- Error messages ---
cat > "$TN_DIR/error-messages.json" << 'EOF'
{
  "errors": {
    "auth_failed": "Authentication failed. Please check your credentials.",
    "token_expired": "Your session has expired. Please log in again.",
    "permission_denied": "You don't have permission to access this resource.",
    "rate_limited": "Too many requests. Please try again later.",
    "not_found": "The requested resource was not found.",
    "internal_error": "An internal error occurred. Please try again later."
  },
  "error_codes": [
    "ERR_AUTH_001",
    "ERR_AUTH_002",
    "ERR_PERM_001",
    "ERR_RATE_001",
    "ERR_404_001",
    "ERR_500_001"
  ]
}
EOF

# --- Log file ---
cat > "$TN_DIR/application.log" << 'EOF'
2024-01-15 10:00:00 INFO  Application started successfully
2024-01-15 10:00:01 INFO  Connected to database at localhost:5432
2024-01-15 10:00:02 INFO  Cache initialized with 1000 entries
2024-01-15 10:00:03 DEBUG Processing request from 192.168.1.100
2024-01-15 10:00:04 INFO  Request completed in 45ms
2024-01-15 10:00:05 WARN  High memory usage detected: 85%
2024-01-15 10:00:06 INFO  Garbage collection completed
2024-01-15 10:00:07 ERROR Failed to connect to external API: timeout
2024-01-15 10:00:08 INFO  Retry successful after 2 attempts
2024-01-15 10:00:09 INFO  Health check passed
EOF

# --- API response ---
cat > "$TN_DIR/api-response.json" << 'EOF'
{
  "status": "success",
  "data": {
    "user_id": "usr_1234567890abcdef",
    "session_id": "ses_abcdef1234567890",
    "request_id": "req_0987654321fedcba",
    "timestamp": "2024-01-15T10:00:00Z"
  },
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
EOF

# --- OpenAPI spec ---
cat > "$TN_DIR/openapi-spec.yaml" << 'EOF'
openapi: 3.0.0
info:
  title: Example API
  version: 1.0.0
  description: API documentation (no secrets here)

paths:
  /users:
    get:
      summary: List users
      responses:
        '200':
          description: Successful response
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'

components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        email:
          type: string
EOF

# --- Package manifest ---
cat > "$TN_DIR/package.json" << 'EOF'
{
  "name": "example-app",
  "version": "1.0.0",
  "description": "Example application (no secrets)",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "jest",
    "build": "webpack --mode production"
  },
  "dependencies": {
    "express": "^4.18.2",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0",
    "webpack": "^5.88.0"
  },
  "author": "Example Author",
  "license": "MIT"
}
EOF

# --- Requirements file ---
cat > "$TN_DIR/requirements.txt" << 'EOF'
# Python dependencies (no secrets)
# These are package names and versions, not credentials

requests==2.31.0
flask==2.3.3
django==4.2.4
numpy==1.25.2
pandas==2.0.3
pytest==7.4.0
black==23.7.0
flake8==6.1.0
mypy==1.5.0
EOF

# --- Dockerfile ---
cat > "$TN_DIR/Dockerfile" << 'EOF'
# Dockerfile (no secrets)
# Best practice: secrets should be passed at runtime, not in Dockerfile

FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3000

USER node

CMD ["node", "server.js"]
EOF

echo "Updated TN count: $(ls -1 "$TN_DIR" | wc -l) files"

echo ""
echo "Final corpus counts:"
echo "True Positives: $(ls -1 "$TP_DIR" | wc -l) files"
echo "True Negatives: $(ls -1 "$TN_DIR" | wc -l) files"
