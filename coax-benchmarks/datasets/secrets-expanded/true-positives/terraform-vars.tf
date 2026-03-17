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
