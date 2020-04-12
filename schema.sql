DROP TABLE IF EXISTS `email_addresses`;
CREATE TABLE `email_addresses`
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL UNIQUE
);

DROP TABLE IF EXISTS `emails`;
CREATE TABLE `emails`
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_id   TEXT NOT NULL UNIQUE,
    sender      TEXT NOT NULL,
    recipient   TEXT NOT NULL,
    subject     TEXT,
    body        TEXT,
    received_at TEXT NOT NULL,
    FOREIGN KEY (recipient) REFERENCES `email_addresses` (address) ON DELETE CASCADE
);
