-- This file should undo anything in `up.sql`
ALTER TABLE queue ADD COLUMN account_id INT NOT NULL;
ALTER TABLE queue ADD FOREIGN KEY (account_id) REFERENCES account(id);
DROP TABLE IF EXISTS account_queue;