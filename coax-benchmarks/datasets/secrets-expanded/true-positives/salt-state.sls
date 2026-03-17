# Salt state with credentials
# WARNING: Test secrets only

myapp:
  config:
    - aws_access_key: AKIAIOSFODNN7EXAMPLE38
    - aws_secret_key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY38
    - github_token: ghp_1234567890abcdefghij1234567890ABCDEF
    - database_password: SaltPassword123!
