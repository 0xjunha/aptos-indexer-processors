-- tracks multikey layouts for multikey user transactions
CREATE TABLE multi_keys
(
    transaction_version BIGINT NOT NULL,
    transaction_block_height BIGINT NOT NULL,
    type VARCHAR(50) NOT NULL,
    public_key VARCHAR(66) NOT NULL,
    public_key_index BIGINT NOT NULL,
    threshold BIGINT NOT NULL,
    total_public_keys BIGINT NOT NULL,
    is_used BOOLEAN NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- Constraints
    PRIMARY KEY (
      transaction_version,
      public_key_index,
    ),
    CONSTRAINT fk_transaction_versions FOREIGN KEY (transaction_version) REFERENCES transactions (version)
);
CREATE INDEX multi_key_insat_index ON multi_keys (inserted_at);