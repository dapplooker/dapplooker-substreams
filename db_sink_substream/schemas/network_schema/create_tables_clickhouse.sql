-- Create schema
CREATE DATABASE IF NOT EXISTS ethereum;

-- Create blocks table
CREATE TABLE IF NOT EXISTS ethereum.blocks (
    id          Int32 NOT NULL,
    hash        String,
    parent_hash String,
    gas_limit   Int64,
    gas_used    Int64,
    timestamp   DateTime,
    size        Int32,
    nonce       String,
    INDEX idx_eth_blk_details_hash (hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_blk_details_timestamp (timestamp) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
PRIMARY KEY (id)
ORDER BY (id, timestamp);

-- Create cursors table
CREATE TABLE IF NOT EXISTS ethereum.cursors (
    id        String NOT NULL,
    cursor    String,
    block_num Int64,
    block_id  String
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id);


-- Create transactions table
CREATE TABLE IF NOT EXISTS ethereum.transactions (
    id                       String NOT NULL,
    status                   String,
    amount                   Decimal(50, 10),
    gas_used                 Int64,
    gas_limit                Int64,
    block_number            Int64,
    gas_price               Int64,
    timestamp               DateTime,
    to_address              String,
    from_address            String,
    max_fee_per_gas         Int64,
    max_priority_fee_per_gas Int64,
    nonce                   String,
    INDEX idx_eth_tx_block_number  (block_number) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_tx_block_timestamp  (timestamp) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_tx_nonce  (nonce) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id, timestamp);


-- Create contracts table
CREATE TABLE IF NOT EXISTS ethereum.contracts (
    id               String NOT NULL,
    block_number     Int64,
    owner            String,
    transaction_hash String NOT NULL,
    timestamp        DateTime,
    INDEX idx_contract_block_number (block_number) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_block_timestamp (timestamp) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_transaction_hash (transaction_hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_id (id) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id, timestamp);
