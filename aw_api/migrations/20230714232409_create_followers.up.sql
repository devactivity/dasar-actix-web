-- Add up migration script here
CREATE TABLE followers (
    user_id UUID NOT NULL REFERENCES users (id),
    follower_id UUID NOT NULL REFERENCES users (id),
    PRIMARY KEY (user_id, follower_id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX followers_user_id_idx ON followers (user_id);
CREATE INDEX followers_follower_user_id_idx ON followers (follower_id);

ALTER TABLE followers ADD CONSTRAINT user_id_cannot_be_equal_to_follower_id_chk CHECK (user_id != follower_id);

SELECT sqlx_manage_updated_at('followers');