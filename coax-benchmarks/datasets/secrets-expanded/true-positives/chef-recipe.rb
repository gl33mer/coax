# Chef recipe with credentials
# WARNING: Test secrets only

node.default['aws']['access_key'] = 'AKIAIOSFODNN7EXAMPLE36'
node.default['aws']['secret_key'] = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY36'
node.default['github']['token'] = 'ghp_1234567890abcdefghij1234567890ABCDEF'
node.default['database']['password'] = 'ChefPassword123!'
