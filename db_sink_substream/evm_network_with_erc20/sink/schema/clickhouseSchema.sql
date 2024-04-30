-- Create schema
CREATE DATABASE IF NOT EXISTS erc20;

-- Create cursors table
CREATE TABLE IF NOT EXISTS erc20.cursors (
    id        String NOT NULL,
    cursor    String,
    block_num Int64,
    block_id  String
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id);

CREATE TABLE IF NOT EXISTS erc20.token (
    name           String,
    decimals           String,
    symbol           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (name);

CREATE TABLE IF NOT EXISTS erc20.account (
    account           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (account);

CREATE TABLE IF NOT EXISTS erc20.balance (  
    token           FixedString(85),
    owner           FixedString(42),
    balance         String
) ENGINE = ReplacingMergeTree()
ORDER BY (token);

CREATE TABLE IF NOT EXISTS erc20.blocks (
    hash           String,
    parent_hash           String,
    timestamp           String,
    size           String,
    nonce           String,
    gas_limit           String,
    gas_used           String
) ENGINE = ReplacingMergeTree()
ORDER BY (hash);

CREATE TABLE IF NOT EXISTS erc20.transactions (
    status           String,
    gas_used           String,
    gas_limit           String,
    gas_price           String,
    nonce           String,
    to_address           String,
    from_address           String,
    max_fee_per_gas           String,
    max_priority_fee_per_gas           String,
    block_number           String,
    timestamp           String,
    amount           String
) ENGINE = ReplacingMergeTree()
ORDER BY (block_number);

CREATE TABLE IF NOT EXISTS erc20.contracts (
    id           String,
    transaction_hash           String,
    owner           String,
    size           String,
    timestamp           String
) ENGINE = ReplacingMergeTree()
ORDER BY (id);