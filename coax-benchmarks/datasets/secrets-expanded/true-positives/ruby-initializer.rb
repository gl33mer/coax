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
