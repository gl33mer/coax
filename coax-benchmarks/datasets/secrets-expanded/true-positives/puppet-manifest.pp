# Puppet manifest with secrets
# WARNING: Test secrets only

$aws_access_key = 'AKIAIOSFODNN7EXAMPLE37'
$aws_secret_key = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY37'
$github_token = 'ghp_1234567890abcdefghij1234567890ABCDEF'
$database_password = 'PuppetPassword123!'

class myapp {
  # Use credentials
}
