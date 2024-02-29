-- Your SQL goes here
CREATE TABLE IF NOT EXISTS matches (
    id bigint not null AUTO_INCREMENT,
    gamemode int not null,
    PRIMARY KEY ( id )
);