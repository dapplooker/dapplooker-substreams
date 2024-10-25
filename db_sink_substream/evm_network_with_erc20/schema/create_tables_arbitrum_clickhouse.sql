-- Create schema
CREATE DATABASE IF NOT EXISTS arbitrum_network;

-- Create cursors table
CREATE TABLE IF NOT EXISTS arbitrum_network.cursors (
    id        String NOT NULL,
    cursor    String,
    block_num Int64,
    block_id  String
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id);

CREATE TABLE IF NOT EXISTS arbitrum_network.token (
    name           String,
    decimals           String,
    symbol           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (name);

CREATE TABLE IF NOT EXISTS arbitrum_network.account (
    account           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (account);

CREATE TABLE IF NOT EXISTS arbitrum_network.balance (  
    token           FixedString(85),
    owner           FixedString(42),
    balance         String
) ENGINE = ReplacingMergeTree()
ORDER BY (token);

-- Create blocks table
CREATE TABLE IF NOT EXISTS arbitrum_network.blocks (
    id          Int32 NOT NULL,
    hash        FixedString(66),
    parent_hash FixedString(66),
    gas_limit   Int64,
    gas_used    Int64,
    timestamp   DateTime,
    size        Int32,
    nonce       String,
    INDEX idx_eth_blk_details_hash (hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_blk_details_timestamp (timestamp) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
ORDER BY (id);

-- Create transactions table
CREATE TABLE IF NOT EXISTS arbitrum_network.transactions (
    id                       FixedString(66) NOT NULL,
    status                   FixedString(1),
    amount                   Float64,
    gas_used                 Int64,
    gas_limit                Int64,
    block_number             Int64,
    gas_price                Int64,
    timestamp                DateTime,
    to_address               FixedString(42),
    from_address             FixedString(42),
    max_fee_per_gas          Int64,
    max_priority_fee_per_gas Int64,
    nonce                    String,
    INDEX idx_eth_tx_block_number  (block_number) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_tx_block_timestamp  (timestamp) TYPE minmax GRANULARITY 8192,
    INDEX idx_eth_tx_nonce  (nonce) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
ORDER BY (id);


-- Create contracts table
CREATE TABLE IF NOT EXISTS arbitrum_network.contracts (
    id               FixedString(42) NOT NULL,
    block_number     Int64,
    owner            FixedString(42),
    transaction_hash FixedString(66) NOT NULL,
    timestamp        DateTime,
    INDEX idx_contract_block_number (block_number) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_block_timestamp (timestamp) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_transaction_hash (transaction_hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_contract_id (id) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
ORDER BY (id);
