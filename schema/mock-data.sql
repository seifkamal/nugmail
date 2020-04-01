DELETE FROM `email_addresses`;
INSERT INTO `email_addresses` (address)
VALUES ('toasty@test.com'),
       ('zeby@test.com');

DELETE FROM `emails`;
INSERT INTO `emails` (sender, recipient, subject, message, received_at)
VALUES ('123@gmail.com', 'toasty@test.com', '<3<3', 'love you toasty', "2020-04-01 18:00:00.000"),
       ('456@gmail.com', 'toasty@test.com', 'hello world', 'dear toasty...', "2020-04-01 18:00:00.000"),
       ('123@gmail.com', 'toasty@test.com', ':*', 'toasty pls', "2020-04-01 18:00:00.000"),
       ('123@gmail.com', 'zeby@test.com', 'die', 'zeby is shit', "2020-04-01 18:00:00.000"),
       ('789@gmail.com', 'zeby@test.com', 'bread?', 'ugh', "2020-04-01 18:00:00.000"),
       ('123@gmail.com', 'zeby@test.com', 'DIE', 'toasty is better than you', "2020-04-01 18:00:00.000"),
       ('789@gmail.com', 'toasty@test.com', 'bread?', 'what are you', "2020-04-01 18:00:00.000")
