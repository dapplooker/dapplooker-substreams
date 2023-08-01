CREATE SCHEMA ethereum;

CREATE TABLE ethereum.blocks (
    id          int not null constraint block_meta_pk primary key,
    hash      text,
    parent_hash text,
    gas_limit bigint,
    gas_used bigint,
    timestamp   timestamp,
    size        int,
    nonce       text
);

CREATE INDEX idx_eth_blk_details_hash ON ethereum.blocks (hash);
CREATE INDEX idx_eth_blk_details_timestamp ON ethereum.blocks (timestamp);


CREATE TABLE ethereum.cursors (
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);


CREATE TABLE ethereum.transactions (
    id         text not null constraint transactions_pk primary key,
    status     text,
    gas_used   bigint,
    gas_limit  bigint,
    block_number bigint,
    gas_price bigint,
    timestamp  timestamp,
    to_address text,
    from_address   text,
    max_fee_per_gas bigint,
    max_priority_fee_per_gas bigint,
    nonce      text
);

CREATE INDEX idx_eth_tx_block_number ON ethereum.transactions (block_number);
CREATE INDEX idx_eth_tx_block_timestamp ON ethereum.transactions (timestamp);
CREATE INDEX idx_eth_tx_nonce ON ethereum.transactions (nonce);


CREATE TABLE ethereum.contracts (
    id         text not null constraint contracts_pk primary key,
    block_number bigint,
    owner        text,
    transaction_hash text,
    timestamp      timestamp
);

CREATE INDEX idx_contract_block_number ON ethereum.contracts (block_number);
CREATE INDEX idx_contract_block_timestamp ON ethereum.contracts (timestamp);
CREATE INDEX idx_contract_transaction_hash ON ethereum.contracts (transaction_hash);
