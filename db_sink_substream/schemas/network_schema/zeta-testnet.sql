CREATE DATABASE IF NOT EXISTS zeta_testnet;


CREATE TABLE IF NOT EXISTS zeta_testnet.transactions
(
    ethereum_tx_hash FixedString(80),
    block_number     Int64,
    tx_index         bigint,
    tx_gas_used      Float64,
    amount           Float64,
    recipient        FixedString(60),
    sender           FixedString(60),
    module           FixedString(20),
    action           FixedString(70),
    date             DateTime,
    INDEX idx_transactions_hash (hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_transactions_date (date) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
ORDER BY (ethereum_tx_hash);

CREATE TABLE IF NOT EXISTS zeta_testnet.blocks (
    block_number     Int64,
    block_hash       FixedString(70),
    parent_hash      FixedString(70),
    proposer_address FixedString(50),
    date             DateTime,
    gas_used         Float64,
    gas_limit        Float64,
    base_fee         Float64,
    burnt_fee        Float64,
    size             Int32,
    INDEX idx_block_hash (hash) TYPE minmax GRANULARITY 8192,
    INDEX idx_block_timestamp (date) TYPE minmax GRANULARITY 8192
) ENGINE = ReplacingMergeTree()
ORDER BY (block_number);

CREATE TABLE IF NOT EXISTS zeta_testnet.validator_details
(
    operator_address           FixedString(60) not null,
    validator_address          FixedString(50) not null,
    consensus_pubkey           FixedString(50),
    jailed                     boolean,
    status                     FixedString(30),
    tokens                     Float64,
    delegator_shares           Float64,
    moniker                    FixedString(60),
    unbonding_height           Float64,
    unbonding_time             DateTime,
    commission_rate            Float64,
    commission_max_rate        Float64,
    commission_max_change_rate Float64,
    commission_update_time     DateTime,
    min_self_delegation        bigint,
    self_bond_rewards          Float64,
    commission                 Float64
) ENGINE = ReplacingMergeTree()
ORDER BY (operator_address);

CREATE TABLE IF NOT EXISTS zeta_testnet.validators
(
    address           FixedString(50),
    block_number      Int64,
    pub_key_type      FixedString(30),
    pub_key_value     FixedString(50),
    voting_power      Float64,
    proposer_priority Float64
) ENGINE = ReplacingMergeTree()
ORDER BY (address);
