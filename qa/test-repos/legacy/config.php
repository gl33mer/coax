<?php
/**
 * Legacy PHP Configuration
 * Old-style configuration file with hardcoded secrets
 * 
 * WARNING: Contains intentional secrets for testing.
 */

// Database Configuration (CRITICAL - should be detected)
$db_host = "localhost";
$db_user = "admin";
$db_password = "SuperSecretPassword123!";
$db_name = "legacy_app";

// MySQL Connection String (CRITICAL - should be detected)
$mysql_connection = "mysql://admin:SuperSecretPassword123!@localhost:3306/legacy_app";

// API Keys (CRITICAL - should be detected)
$google_api_key = "AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe";
$sendgrid_key = "SG.abcdefghijklmnopqrstuv.1234567890abcdefghijklmnopqrstuvwxyzABCDEF";

// Legacy password storage (bad practice, but should be detected)
$password = "admin123";
$secret = "my-super-secret-key-12345";

// Database connection function
function getDatabaseConnection() {
    global $db_host, $db_user, $db_password, $db_name;
    return new mysqli($db_host, $db_user, $db_password, $db_name);
}

// Legacy authentication (insecure, but for testing)
function authenticate($username, $password) {
    // Hardcoded admin credentials (CRITICAL - should be detected)
    if ($username === "admin" && $password === "admin123") {
        return true;
    }
    return false;
}
?>
