-- Add migration script here
CREATE TABLE user_balance (
    balance_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    token_name VARCHAR(50),
    amount NUMERIC(18, 8) DEFAULT 0.00,

    CONSTRAINT fk_user
    FOREIGN  KEY (user_id)
    REFERENCES users(id)
    
);