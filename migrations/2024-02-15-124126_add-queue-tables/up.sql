-- Your SQL goes here
CREATE TABLE IF NOT EXISTS queue (
    id bigint not null AUTO_INCREMENT,
    gamemode smallint not null,
    account_id int not null,
    PRIMARY KEY (id),
    FOREIGN KEY (account_id) REFERENCES account(id)
);

CREATE TABLE IF NOT EXISTS c2_match (
    id bigint not null AUTO_INCREMENT,
    gamemode smallint not null,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS account_match (
    id bigint not null AUTO_INCREMENT,
    match_id bigint not null,
    account_id int not null,
    PRIMARY KEY (id),
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (match_id) REFERENCES c2_match(id)
);