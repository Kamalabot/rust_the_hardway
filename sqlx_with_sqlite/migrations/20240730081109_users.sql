CREATE TABLE IF NOT EXISTS accounts 
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);