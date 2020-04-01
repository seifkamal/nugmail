DROP TABLE IF EXISTS `email_addresses`;
CREATE TABLE `email_addresses`
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT UNIQUE NOT NULL
);

DROP TABLE IF EXISTS `emails`;
CREATE TABLE `emails`
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    sender      TEXT NOT NULL,
    recipient   TEXT NOT NULL,
    subject     TEXT,
    message     TEXT,
    received_at TEXT NOT NULL,
    FOREIGN KEY (recipient) REFERENCES `email_addresses` (address) ON DELETE CASCADE
);
