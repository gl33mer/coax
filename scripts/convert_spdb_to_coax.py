#!/usr/bin/env python3
"""
Convert secrets-patterns-db patterns to Coax YAML format.

This script reads patterns from the secrets-patterns-db repository
and converts them to Coax's YAML pattern format.

Usage:
    python convert_spdb_to_coax.py <input_file> <output_file>

Example:
    python convert_spdb_to_coax.py /tmp/secrets-patterns-db/db/rules-stable.yml config/patterns/secrets_patterns_db.yml
"""

import sys
import re
from pathlib import Path


def parse_spdb_yaml(content: str) -> list:
    """Simple parser for secrets-patterns-db YAML format."""
    patterns = []
    current_pattern = None
    current_section = None
    
    for line in content.split('\n'):
        # Skip comments and empty lines
        if line.strip().startswith('#') or not line.strip():
            continue
        
        # Check for pattern start
        if '- pattern:' in line:
            if current_pattern:
                patterns.append(current_pattern)
            current_pattern = {}
            current_section = 'pattern'
            continue
        
        # Check for nested fields
        if current_pattern is not None:
            # Match "name: value" or "regex: value" or "confidence: value"
            match = re.match(r'\s{6,8}(name|regex|confidence):\s*(.+)', line)
            if match:
                key, value = match.groups()
                # Remove quotes if present
                value = value.strip().strip('"\'')
                current_pattern[key] = value
    
    if current_pattern:
        patterns.append(current_pattern)
    
    return patterns


def categorize_pattern(name: str) -> str:
    """Categorize a pattern based on its name."""
    name_lower = name.lower()
    
    # Cloud providers
    if any(x in name_lower for x in ['aws', 'amazon', 'gcp', 'google', 'azure', 'digitalocean', 'heroku', 'linode']):
        return 'cloud_provider'
    
    # Version control
    if any(x in name_lower for x in ['github', 'gitlab', 'bitbucket', 'gitea']):
        return 'version_control'
    
    # Payment
    if any(x in name_lower for x in ['stripe', 'square', 'paypal', 'braintree', 'adyen', 'plaid']):
        return 'payment'
    
    # Communication
    if any(x in name_lower for x in ['slack', 'sendgrid', 'twilio', 'mailgun', 'discord', 'telegram', 'whatsapp']):
        return 'communication'
    
    # Database
    if any(x in name_lower for x in ['postgresql', 'postgres', 'mongodb', 'mysql', 'redis', 'mssql', 'oracle', 'cassandra', 'elasticsearch', 'databricks']):
        return 'database'
    
    # Private keys
    if any(x in name_lower for x in ['rsa', 'ec ', 'dsa', 'ssh', 'pgp', 'private key', 'certificate']):
        return 'private_key'
    
    # AI/ML
    if any(x in name_lower for x in ['openai', 'anthropic', 'huggingface', 'cohere', 'ai21', 'replicate', 'stability', 'together', 'perplexity']):
        return 'ai_ml'
    
    # API keys (generic)
    if any(x in name_lower for x in ['api', 'token', 'key', 'secret', 'access', 'auth']):
        return 'api_key'
    
    return 'generic'


def determine_severity(confidence: str, name: str) -> str:
    """Determine severity based on confidence and pattern name."""
    name_lower = name.lower()
    
    # Critical for private keys and high-value targets
    if any(x in name_lower for x in ['private key', 'ssh', 'rsa', 'aws', 'stripe', 'payment']):
        return 'critical'
    
    # High for most API keys
    if confidence == 'high':
        return 'high'
    
    # Medium for lower confidence
    if confidence == 'low':
        return 'medium'
    
    return 'medium'


def generate_recommendation(name: str, category: str) -> str:
    """Generate a recommendation based on pattern name and category."""
    name_lower = name.lower()
    
    recommendations = {
        'aws': 'Remove immediately and rotate the key via AWS IAM Console',
        'github': 'Remove and regenerate the token in GitHub Settings',
        'gitlab': 'Remove and revoke the GitLab token',
        'google': 'Remove and restrict API key usage in Google Cloud Console',
        'azure': 'Remove and regenerate key in Azure Portal',
        'stripe': 'Remove and rotate immediately in Stripe Dashboard',
        'slack': 'Remove and revoke the token in Slack Admin',
        'sendgrid': 'Remove and regenerate in SendGrid Settings',
        'twilio': 'Remove and revoke in Twilio Console',
        'discord': 'Remove and regenerate in Discord Developer Portal',
        'telegram': 'Remove and revoke bot via @BotFather',
        'mongodb': 'Remove and rotate MongoDB credentials',
        'postgresql': 'Remove and rotate database credentials',
        'mysql': 'Remove and rotate MySQL credentials',
        'redis': 'Remove and change Redis AUTH password',
        'openai': 'Remove and revoke in OpenAI Platform',
        'anthropic': 'Remove and revoke in Anthropic Console',
        'huggingface': 'Remove and delete in Hugging Face Settings',
        'private key': 'Remove immediately - private key exposed',
        'ssh': 'Remove and revoke SSH key from all servers',
    }
    
    for key, rec in recommendations.items():
        if key in name_lower:
            return rec
    
    return 'Review this finding and rotate credentials if valid'


def escape_yaml_string(s: str) -> str:
    """Escape a string for YAML output."""
    # Escape backslashes first
    s = s.replace('\\', '\\\\')
    # Use single quotes and escape single quotes inside
    if "'" in s:
        s = s.replace("'", "''")  # YAML escapes single quotes by doubling
        return f"'{s}'"
    return f"'{s}'"


def convert_spdb_to_coax(input_file: str, output_file: str, min_confidence: str = 'high'):
    """Convert secrets-patterns-db format to Coax YAML format."""
    
    # Read input file
    with open(input_file, 'r') as f:
        content = f.read()
    
    patterns = parse_spdb_yaml(content)
    
    # Filter by confidence
    confidence_order = {'high': 2, 'medium': 1, 'low': 0}
    min_conf_level = confidence_order.get(min_confidence, 1)
    
    coax_patterns = []
    for pattern in patterns:
        confidence = pattern.get('confidence', 'medium')
        
        # Skip if below minimum confidence
        if confidence_order.get(confidence, 0) < min_conf_level:
            continue
        
        name = pattern.get('name', 'UNKNOWN')
        regex = pattern.get('regex', '')
        
        # Skip patterns with invalid regex characters
        if not regex or regex.startswith('^') or regex.endswith('$'):
            # These are typically too broad or anchored
            continue
        
        category = categorize_pattern(name)
        severity = determine_severity(confidence, name)
        recommendation = generate_recommendation(name, category)
        
        coax_patterns.append({
            'name': name,
            'regex': regex,
            'severity': severity,
            'recommendation': recommendation,
            'description': name.replace('_', ' ').title(),
            'cwe_id': 'CWE-798' if severity in ['critical', 'high'] else None,
            'confidence': confidence,
            'category': category,
            'enabled': True
        })
    
    # Write output file
    with open(output_file, 'w') as f:
        f.write("# Coax Patterns imported from secrets-patterns-db\n")
        f.write("# License: CC BY-SA 4.0\n")
        f.write("# Source: https://github.com/mazen160/secrets-patterns-db\n")
        f.write(f"# Total patterns: {len(coax_patterns)} (filtered by confidence: {min_confidence})\n")
        f.write(f"# Generated: {Path(input_file).name}\n\n")
        f.write("patterns:\n")
        
        for p in coax_patterns:
            f.write(f"  - name: {escape_yaml_string(p['name'])}\n")
            f.write(f"    regex: {escape_yaml_string(p['regex'])}\n")
            f.write(f"    severity: {p['severity']}\n")
            f.write(f"    recommendation: {escape_yaml_string(p['recommendation'])}\n")
            f.write(f"    description: {escape_yaml_string(p['description'])}\n")
            if p['cwe_id']:
                f.write(f"    cwe_id: {p['cwe_id']}\n")
            f.write(f"    confidence: {p['confidence']}\n")
            f.write(f"    category: {p['category']}\n")
            f.write(f"    enabled: {str(p['enabled']).lower()}\n")
            f.write("\n")
    
    print(f"Converted {len(coax_patterns)} patterns to {output_file}")
    
    # Print category breakdown
    categories = {}
    for p in coax_patterns:
        cat = p['category']
        categories[cat] = categories.get(cat, 0) + 1
    
    print("\nCategory breakdown:")
    for cat, count in sorted(categories.items(), key=lambda x: -x[1]):
        print(f"  {cat}: {count}")


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("Usage: python convert_spdb_to_coax.py <input_file> <output_file> [min_confidence]")
        print("  min_confidence: high, medium, or low (default: high)")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    min_confidence = sys.argv[3] if len(sys.argv) > 3 else 'high'
    
    convert_spdb_to_coax(input_file, output_file, min_confidence)
