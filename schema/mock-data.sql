DELETE FROM `email_addresses`;
INSERT INTO `email_addresses` (address)
VALUES ('5cd773bf-e6a6-4681-aabf-dff604766f53@test.com'),
       ('34ce9ab3-c547-4155-b2ee-0b7c8161015e@test.com');

DELETE FROM `emails`;
INSERT INTO `emails` (remote_id, sender, recipient, subject, body, received_at)
VALUES ('abc', '123@gmail.com', '5cd773bf-e6a6-4681-aabf-dff604766f53@test.com', '<3<3', 'love you toasty', '2020-04-01 18:00:00.000'),
       ('def', '456@gmail.com', '5cd773bf-e6a6-4681-aabf-dff604766f53@test.com', 'hello world', 'dear toasty...', '2020-04-01 18:00:00.000'),
       ('ghi', '123@gmail.com', '5cd773bf-e6a6-4681-aabf-dff604766f53@test.com', ':*', 'toasty pls', '2020-04-01 18:00:00.000'),
       ('jkl', '123@gmail.com', '34ce9ab3-c547-4155-b2ee-0b7c8161015e@test.com', 'die', 'zeby is shit', '2020-04-01 18:00:00.000'),
       ('mno', '789@gmail.com', '34ce9ab3-c547-4155-b2ee-0b7c8161015e@test.com', 'bread?', 'ugh', '2020-04-01 18:00:00.000'),
       ('pqr', '123@gmail.com', '34ce9ab3-c547-4155-b2ee-0b7c8161015e@test.com', 'DIE', 'toasty is better than you', '2020-04-01 18:00:00.000'),
       ('stu', '789@gmail.com', '5cd773bf-e6a6-4681-aabf-dff604766f53@test.com', 'bread?', 'what are you', '2020-04-01 18:00:00.000')
