# Example Configuration

This is an example configuration file for documentation purposes.

## AWS Configuration

```yaml
aws:
  access_key: "AKIAIOSFODNN7EXAMPLE"
  secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
  region: us-east-1
```

## API Configuration

```yaml
api:
  key: "your-api-key-here"
  secret: "CHANGEME"
  token: "xxx"
```

## Database Configuration

```yaml
database:
  url: "postgresql://user:password@localhost/mydb"
  # Remember to replace with your actual credentials
```

## Notes

- Replace `your-api-key-here` with your actual API key
- Replace `CHANGEME` with a secure password
- Never commit real secrets to version control
- Use environment variables for production secrets
