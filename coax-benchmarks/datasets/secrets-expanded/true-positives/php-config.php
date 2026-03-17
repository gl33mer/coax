<?php
// PHP application configuration
// WARNING: Test secrets only

return [
    'database' => [
        'host' => 'db.example.com',
        'username' => 'admin',
        'password' => 'PhpDatabasePassword123!',
    ],
    
    'aws' => [
        'key' => 'AKIAIOSFODNN7EXAMPLE7',
        'secret' => 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY7',
    ],
    
    'stripe' => [
        'secret' => 'sk_live_1234567890abcdefghijklmnopqrstuv',
    ],
];
