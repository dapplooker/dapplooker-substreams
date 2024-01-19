-- Create schema
CREATE DATABASE IF NOT EXISTS ethereum;

-- Create blocks table
CREATE TABLE IF NOT EXISTS ethereum.blocks (
    id          Int32 NOT NULL PRIMARY KEY,
    hash        String,
    parent_hash String,
    gas_limit   Int64,
    gas_used    Int64,
    timestamp   DateTime,
    size        Int32,
    nonce       String
) ENGINE = MergeTree()
    ORDER BY (id);

-- Create indices for blocks table
CREATE INDEX IF NOT EXISTS idx_eth_blk_details_hash ON ethereum.blocks (hash) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_eth_blk_details_timestamp ON ethereum.blocks (timestamp)  TYPE minmax GRANULARITY 1;


-- Create cursors table
CREATE TABLE IF NOT EXISTS ethereum.cursors (
    id        String NOT NULL PRIMARY KEY,
    cursor    String,
    block_num Int64,
    block_id  String
) ENGINE = MergeTree()
    ORDER BY (id);


-- Create transactions table
CREATE TABLE IF NOT EXISTS ethereum.transactions (
    id                       String NOT NULL PRIMARY KEY,
    status                   String,
    amount                   Decimal,
    gas_used                 Int64,
    gas_limit                Int64,
    block_number            Int64,
    gas_price               Int64,
    timestamp               DateTime,
    to_address              String,
    from_address            String,
    max_fee_per_gas         Int64,
    max_priority_fee_per_gas Int64,
    nonce                   Decimal
) ENGINE = MergeTree()
    ORDER BY (id);

-- Create indices for transactions table
CREATE INDEX IF NOT EXISTS idx_eth_tx_block_number ON ethereum.transactions (block_number) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_eth_tx_block_timestamp ON ethereum.transactions (timestamp) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_eth_tx_nonce ON ethereum.transactions (nonce) TYPE minmax GRANULARITY 1;


-- Create contracts table
CREATE TABLE IF NOT EXISTS ethereum.contracts (
    id               String NOT NULL,
    block_number     Int64,
    owner            String,
    transaction_hash String NOT NULL,
    timestamp        DateTime
) ENGINE = MergeTree()
    ORDER BY (id);

-- Create indices for contracts table
CREATE INDEX IF NOT EXISTS idx_contract_block_number ON ethereum.contracts (block_number) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_contract_block_timestamp ON ethereum.contracts (timestamp) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_contract_transaction_hash ON ethereum.contracts (transaction_hash) TYPE minmax GRANULARITY 1;
CREATE INDEX IF NOT EXISTS idx_contract_id ON ethereum.contracts (id) TYPE minmax GRANULARITY 1;
