# Makefile with credentials
# WARNING: Test secrets only

AWS_ACCESS_KEY_ID = AKIAIOSFODNN7EXAMPLE40
AWS_SECRET_ACCESS_KEY = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY40
GITHUB_TOKEN = ghp_1234567890abcdefghij1234567890ABCDEF
DATABASE_PASSWORD = MakefilePassword123!

deploy:
	@echo "Deploying with AWS key: $(AWS_ACCESS_KEY_ID)"
	@aws s3 sync ./dist s3://my-bucket/
