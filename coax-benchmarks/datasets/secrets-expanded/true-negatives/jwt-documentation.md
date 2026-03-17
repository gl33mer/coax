# JWT Token Format

A JWT token consists of three parts separated by dots:

```
header.payload.signature
```

## Example Format

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

## Parts Explained

1. **Header**: Contains token type and algorithm
   - Example: `{"alg": "HS256", "typ": "JWT"}`

2. **Payload**: Contains claims (user data)
   - Example: `{"sub": "1234567890", "name": "John Doe", "iat": 1516239022}`

3. **Signature**: Verifies token integrity
   - Created using header, payload, and secret

## Important

This is documentation only. The example token above is for illustration.
Never share real JWT tokens in documentation.
