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

CREATE TABLE ethereum.contracts (
    id         text not null constraint contracts_pk primary key,
    block_number bigint,
    owner        text,
    transaction_hash text,
    timestamp      timestamp   
);
