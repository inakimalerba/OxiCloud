-- Script to safely reset the admin user in OxiCloud
-- Run this script to delete the existing admin user if you're having issues creating one

-- Set the correct schema
SET search_path TO auth;

-- Delete the admin user if it exists
DELETE FROM auth.users WHERE username = 'admin';

-- Check if the user was deleted
SELECT 'Admin user has been removed successfully. You can now create a new admin user.' AS message 
WHERE NOT EXISTS (SELECT 1 FROM auth.users WHERE username = 'admin');

-- Check if there are still users in the system
SELECT 'Warning: No users remain in the system. You should register a new admin user.' AS warning 
WHERE NOT EXISTS (SELECT 1 FROM auth.users LIMIT 1);

-- Output remaining users for verification
SELECT username, email, role FROM auth.users ORDER BY role, username;