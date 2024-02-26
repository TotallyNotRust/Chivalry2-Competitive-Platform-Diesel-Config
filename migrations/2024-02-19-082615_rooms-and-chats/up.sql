-- Your SQL goes here
CREATE TABLE IF NOT EXISTS room (
    id BIGINT NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS chat (
    id BIGINT NOT NULL AUTO_INCREMENT,
    chat_text VARCHAR(255) NOT NULL,
    sent_by_id INT NOT NULL,
    room_id BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (sent_by_id) REFERENCES account(id),
    FOREIGN KEY (room_id) REFERENCES room(id)
);

-- Used to give account access to a room, essentially many to many between room and account
CREATE TABLE IF NOT EXISTS room_access (
    id BIGINT NOT NULL AUTO_INCREMENT,
    account_id INT NOT NULL,
    room_id BIGINT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE,
    FOREIGN KEY (room_id) REFERENCES room(id)
);