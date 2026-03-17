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
