#!/usr/bin/env python3
"""Generate ground-truth.yaml for expanded benchmark corpus."""

import os
import yaml

TP_DIR = "coax-benchmarks/datasets/secrets-expanded/true-positives"
TN_DIR = "coax-benchmarks/datasets/secrets-expanded/true-negatives"
OUTPUT_FILE = "coax-benchmarks/datasets/secrets-expanded/ground-truth.yaml"

# Secret type mappings based on filename patterns
SECRET_TYPE_MAP = {
    'aws': 'aws_credentials',
    'github': 'github_tokens',
    'stripe': 'stripe_keys',
    'datadog': 'datadog_keys',
    'twilio': 'twilio_credentials',
    'slack': 'slack_tokens',
    'sendgrid': 'sendgrid_api_key',
    'mailgun': 'mailgun_api_key',
    'discord': 'discord_token',
    'telegram': 'telegram_bot_token',
    'openai': 'openai_api_key',
    'anthropic': 'anthropic_api_key',
    'npm': 'npm_token',
    'pypi': 'pypi_token',
    'database': 'database_credentials',
    'postgresql': 'database_credentials',
    'mongodb': 'database_credentials',
    'redis': 'database_credentials',
    'private_key': 'private_key',
    'ssh': 'private_key',
    'jwt': 'jwt_token',
    'azure': 'azure_credentials',
    'gcp': 'gcp_credentials',
    'google': 'gcp_credentials',
}

def get_secret_type(filename):
    """Determine secret type from filename."""
    filename_lower = filename.lower()
    for pattern, secret_type in SECRET_TYPE_MAP.items():
        if pattern in filename_lower:
            return secret_type
    
    # Default based on file extension
    ext = os.path.splitext(filename)[1].lower()
    if ext in ['.env', '.env.*']:
        return 'environment_secrets'
    elif ext in ['.yml', '.yaml']:
        return 'config_secrets'
    elif ext in ['.json']:
        return 'config_secrets'
    elif ext in ['.py', '.js', '.ts', '.go', '.rs', '.java', '.cs', '.rb', '.php']:
        return 'code_secrets'
    elif ext in ['.tf', '.tfvars']:
        return 'infrastructure_secrets'
    elif ext in ['.sh', '.bash', '.ps1', '.bat', '.cmd']:
        return 'script_secrets'
    elif ext in ['.yml', '.yaml'] and 'ci' in filename_lower or 'workflow' in filename_lower:
        return 'ci_cd_secrets'
    elif ext in ['.gradle', '.pom', '.properties']:
        return 'build_secrets'
    elif ext in ['.npmrc', '.pypirc', '.netrc']:
        return 'package_registry_secrets'
    elif ext in ['.exs', '.config', '.pp', '.sls']:
        return 'config_secrets'
    elif ext in ['.R', '.r', '.m', '.ml', '.mli']:
        return 'code_secrets'
    elif ext in ['.pl', '.pm']:
        return 'script_secrets'
    elif ext in ['.lua']:
        return 'config_secrets'
    elif ext in ['.xml']:
        return 'config_secrets'
    elif ext in ['.toml']:
        return 'config_secrets'
    elif ext in ['.csv']:
        return 'data_dump_secrets'
    elif ext in ['.sql']:
        return 'database_secrets'
    elif ext in ['.ipynb']:
        return 'notebook_secrets'
    elif ext in ['.md', '.markdown']:
        return 'documentation_secrets'
    elif ext in ['.makefile', 'makefile']:
        return 'build_secrets'
    elif ext in ['.vagrantfile', 'vagrantfile']:
        return 'infrastructure_secrets'
    elif ext in ['.jenkinsfile']:
        return 'ci_cd_secrets'
    
    return 'unknown'

def count_expected_findings(filename):
    """Estimate expected finding count based on file content hints."""
    # This is a rough estimate - actual count may vary
    filename_lower = filename.lower()
    
    # Files with multiple secrets
    if any(x in filename_lower for x in ['env', 'config', 'credentials', 'secrets', 'compose', 'ci', 'jenkins', 'gitlab', 'circle', 'travis']):
        return 4
    elif any(x in filename_lower for x in ['aws', 'github', 'stripe', 'database']):
        return 2
    else:
        return 1

def generate_ground_truth():
    """Generate ground truth manifest."""
    ground_truth = {
        'true_positives': {},
        'true_negatives': {}
    }
    
    # Process true positives
    tp_files = sorted(os.listdir(TP_DIR))
    for filename in tp_files:
        filepath = os.path.join(TP_DIR, filename)
        if os.path.isfile(filepath):
            secret_type = get_secret_type(filename)
            expected_count = count_expected_findings(filename)
            
            ground_truth['true_positives'][filename] = {
                'description': f'Test file containing {secret_type}',
                'secret_types': [secret_type],
                'expected_finding_count': expected_count,
                'should_detect': True
            }
    
    # Process true negatives
    tn_files = sorted(os.listdir(TN_DIR))
    for filename in tn_files:
        filepath = os.path.join(TN_DIR, filename)
        if os.path.isfile(filepath):
            # Determine why this is a true negative
            if 'placeholder' in filename.lower() or 'example' in filename.lower():
                reason = 'Contains placeholder/example values, not real secrets'
            elif 'hash' in filename.lower() or 'checksum' in filename.lower():
                reason = 'Contains hash/checksum values, not secrets'
            elif 'uuid' in filename.lower():
                reason = 'Contains UUIDs, not secrets'
            elif 'base64' in filename.lower():
                reason = 'Contains Base64-encoded non-secret data'
            elif 'encrypted' in filename.lower():
                reason = 'Contains encrypted vault values'
            elif 'doc' in filename.lower() or 'readme' in filename.lower():
                reason = 'Documentation with example formats only'
            elif 'url' in filename.lower() or 'log' in filename.lower():
                reason = 'Contains URL parameters or log entries, not secrets'
            elif 'css' in filename.lower() or 'color' in filename.lower():
                reason = 'Contains CSS hex colors, not secrets'
            elif 'jwt' in filename.lower() and 'doc' in filename.lower():
                reason = 'JWT documentation, not real tokens'
            elif 'minified' in filename.lower():
                reason = 'Minified code with high entropy, not secrets'
            elif 'random' in filename.lower() or 'test_data' in filename.lower():
                reason = 'Random test data generation, not secrets'
            elif 'ssh' in filename.lower() and 'fake' in filename.lower():
                reason = 'Fake SSH key for testing, not real key'
            elif 'ssl' in filename.lower() and 'fake' in filename.lower():
                reason = 'Fake SSL certificate for testing'
            elif 'binary' in filename.lower():
                reason = 'Binary file signatures, not secrets'
            elif 'version' in filename.lower():
                reason = 'Version strings and build IDs, not secrets'
            elif 'feature' in filename.lower() or 'flag' in filename.lower():
                reason = 'Feature flags, not secrets'
            elif 'analytics' in filename.lower():
                reason = 'Analytics tracking IDs, not secrets'
            elif 'error' in filename.lower():
                reason = 'Error messages, not secrets'
            elif 'api-response' in filename.lower() or 'response' in filename.lower():
                reason = 'API response data, not secrets'
            elif 'openapi' in filename.lower() or 'spec' in filename.lower():
                reason = 'API specification, not secrets'
            elif 'package' in filename.lower() and 'json' in filename.lower():
                reason = 'Package manifest, not secrets'
            elif 'requirements' in filename.lower():
                reason = 'Python dependencies, not secrets'
            elif 'dockerfile' in filename.lower():
                reason = 'Docker build instructions, no secrets'
            elif 'application.log' in filename.lower() or 'log' in filename.lower():
                reason = 'Application logs, not secrets'
            else:
                reason = 'Does not contain actual secrets'
            
            ground_truth['true_negatives'][filename] = {
                'description': reason,
                'expected_finding_count': 0,
                'should_detect': False
            }
    
    # Write output
    with open(OUTPUT_FILE, 'w') as f:
        yaml.dump(ground_truth, f, default_flow_style=False, sort_keys=True, allow_unicode=True)
    
    print(f"Generated {OUTPUT_FILE}")
    print(f"  True Positives: {len(ground_truth['true_positives'])} files")
    print(f"  True Negatives: {len(ground_truth['true_negatives'])} files")

if __name__ == '__main__':
    generate_ground_truth()
