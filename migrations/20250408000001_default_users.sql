-- Migration 002: Default Users

-- Check if admin user already exists before creating it
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM auth.users WHERE username = 'admin') THEN
        -- Create admin user (password: Admin123!)
        INSERT INTO auth.users (
            id, 
            username, 
            email, 
            password_hash, 
            role, 
            storage_quota_bytes
        ) VALUES (
            '00000000-0000-0000-0000-000000000000',
            'admin',
            'admin@oxicloud.local',
            '$argon2id$v=19$m=65536,t=3,p=4$c2FsdHNhbHRzYWx0c2FsdA$H3VxE8LL2qPT31DM3loTg6D+O4MSc2sD7GjlQ5h7Jkw', -- Admin123!
            'admin',
            107374182400  -- 100GB for admin
        );
    END IF;
END;
$$;

-- Check if test user already exists before creating it
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM auth.users WHERE username = 'test') THEN
        -- Create test user (password: test123)
        INSERT INTO auth.users (
            id, 
            username, 
            email, 
            password_hash, 
            role, 
            storage_quota_bytes
        ) VALUES (
            '11111111-1111-1111-1111-111111111111',
            'test',
            'test@oxicloud.local',
            '$argon2id$v=19$m=65536,t=3,p=4$c2FsdHNhbHRzYWx0c2FsdA$ZG17Z7SFKhs9zWYbuk08CkHpyiznnZapYnxN5Vi62R4', -- test123
            'user',
            10737418240  -- 10GB for test user
        );
    END IF;
END;
$$;