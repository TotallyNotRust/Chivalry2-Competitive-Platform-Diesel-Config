-- Your SQL goes here
CREATE TABLE IF NOT EXISTS made_matches ( /* match and matches are both keywords so i improvise... */
    id bigint not null AUTO_INCREMENT,
    gamemode int not null,
    PRIMARY KEY ( id )
);