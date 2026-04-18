-- Add migration script here
ALTER TABLE users ADD COLUMN user_name 
VARCHAR(50) UNIQUE NOT NULL;