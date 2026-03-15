# False Positive Test Cases

This directory contains test cases for validating false positive reduction.

## Test Categories

### 1. Placeholder Values (Should NOT be flagged)
- `your-access-key`
- `your-secret-key`
- `xxx`
- `CHANGEME`
- `example`

### 2. AWS Example Keys (Should NOT be flagged)
- `AKIAIOSFODNN7EXAMPLE`
- `wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY`

### 3. Constant Key Names (Should NOT be flagged)
- `SESSION_KEY = "ov_console_api_key"`
- `THEME_MODE_KEY = "ov_console_theme_mode"`
- `NAV_COLLAPSED_KEY = "ov_console_nav_collapsed"`

### 4. Comments (Should NOT be flagged)
- `// AWS_KEY=AKIAIOSFODNN7EXAMPLE`
- `# api_key = "sk_live_1234567890"`

### 5. Test Files (Should NOT be flagged or low severity)
- Files in `tests/` directory
- Files named `*_test.py`, `*.test.js`

### 6. Documentation (Should NOT be flagged)
- `*.md` files
- `*.rst` files

### 7. Real Secrets (SHOULD be flagged)
- Real AWS keys (not example)
- Real GitHub tokens
- Real Stripe keys
