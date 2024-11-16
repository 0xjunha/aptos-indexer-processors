-- tracks multikey layouts for multikey user transactions
CREATE TABLE multikey_layouts (
  transaction_version BIGINT NOT NULL,
  transaction_block_height BIGINT NOT NULL,
  signer VARCHAR(66) NOT NULL,
  type VARCHAR(50) NOT NULL,
  public_key VARCHAR(132) NOT NULL,
  public_key_index BIGINT NOT NULL,
  threshold BIGINT NOT NULL,
  total_public_keys BIGINT NOT NULL,
  is_used BOOLEAN NOT NULL,
  inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Constraints
  PRIMARY KEY (
    transaction_version,
    public_key_index
  )
);
CREATE INDEX multikey_layouts_insat_index ON multikey_layouts (inserted_at);