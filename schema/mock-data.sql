DELETE
FROM `email_addresses`;
INSERT INTO `email_addresses` (address)
VALUES ('toasty@test.com'),
       ('zeby@test.com');

DELETE
FROM `emails`;
INSERT INTO `emails` (remote_id, sender, recipient, subject, body, received_at)
VALUES ('abc', '123@gmail.com', 'toasty@test.com', '<3<3', 'love you toasty', '2020-04-01 18:00:00.000'),
       ('def', '456@gmail.com', 'toasty@test.com', 'hello world', 'dear toasty...', '2020-04-01 18:00:00.000'),
       ('ghi', '123@gmail.com', 'toasty@test.com', ':*', 'toasty pls', '2020-04-01 18:00:00.000'),
       ('jkl', '123@gmail.com', 'zeby@test.com', 'die', 'zeby is shit', '2020-04-01 18:00:00.000'),
       ('mno', '789@gmail.com', 'zeby@test.com', 'bread?', 'ugh', '2020-04-01 18:00:00.000'),
       ('pqr', '123@gmail.com', 'zeby@test.com', 'DIE', 'toasty is better than you', '2020-04-01 18:00:00.000'),
       ('stu', '789@gmail.com', 'toasty@test.com', 'bread?', 'what are you', '2020-04-01 18:00:00.000')
