START TRANSACTION;

CREATE TABLE social_net.messages
(
    user_from       CHAR(36) NOT NULL,
    user_to         CHAR(36) NOT NULL,
    message         VARCHAR(999) NOT NULL,
    ts              TIMESTAMP
);

-- Create an index on user_to field
CREATE INDEX idx_user_to ON social_net.messages (user_to);

COMMIT;