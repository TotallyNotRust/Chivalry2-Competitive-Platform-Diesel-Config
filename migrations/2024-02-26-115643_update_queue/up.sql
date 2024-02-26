-- Your SQL goes here
ALTER TABLE queue DROP FOREIGN KEY queue_ibfk_1;

ALTER TABLE queue DROP COLUMN account_id;

CREATE TABLE IF NOT EXISTS account_queue (
    account_id INT NOT NULL,
    queue_id BIGINT NOT NULL,
    PRIMARY KEY (account_id, queue_id),
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (queue_id) REFERENCES queue(id)
);
