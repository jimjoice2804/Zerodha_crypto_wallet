-- Add migration script here
CREATE TABLE user_transaction (
    id SERIAL UNIQUE NOT NULL PRIMARY KEY,
    sender_id INT NOT NULL,
    receiver_id INT NOT NULL,
    transaction_amount NUMERIC(18,8) NOT NULL,
    time_of_transaction TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 

    CONSTRAINT fk_sender_user
    FOREIGN KEY  (sender_id)
    REFERENCES users(id)
,
    CONSTRAINT fk_receiver_user
    FOREIGN KEY  (receiver_id)
    REFERENCES users(id)
);