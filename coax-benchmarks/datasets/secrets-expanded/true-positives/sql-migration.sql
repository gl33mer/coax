-- Database migration with hardcoded credentials
-- WARNING: Test secrets only

INSERT INTO api_keys (user_id, key_value, created_at)
VALUES 
    (1, 'AKIAIOSFODNN7EXAMPLE14', NOW()),
    (2, 'ghp_1234567890abcdefghij1234567890ABCDEF', NOW()),
    (3, 'sk_live_1234567890abcdefghijklmnopqrstuv', NOW());

-- Update admin password
UPDATE users SET password_hash = 'SqlMigrationPassword123!' WHERE username = 'admin';
