mod pb;
mod db;
mod tables;

use pb::acme;
use crate::tables::Tables;
use pb::acme::{Transaction, TransactionList, BlockHeader, ContractList, Contract};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams::store::{StoreNew, StoreSetProto};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::store::StoreSet;
use hex;    
use substreams_ethereum::pb::eth::v2::BigInt;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_contract(blk: eth::Block) -> Result<ContractList, substreams::errors::Error> {
    let contract_list: Vec<Contract> = blk
        .transaction_traces
        .clone()
        .into_iter()
        .filter(|trx| trx.status == TransactionTraceStatus::Succeeded as i32)
        .filter_map(|trx| process_contract_trace(trx, &blk))
        .collect();

    Ok(ContractList { contract_list })
}

fn process_contract_trace(
    trx: eth::TransactionTrace,
    block: &eth::Block,
) -> Option<Contract> {
    let header = block.header.as_ref().unwrap();
    let contract_check = String::from_utf8_lossy(&trx.input).to_string();
    let block_number = block.number;
    let time_stamp = header.timestamp.clone().unwrap().seconds;
    if contract_check.starts_with("`�`@R") {
        Some(Contract {
            owner: base_64_to_hex(trx.from),
            address: base_64_to_hex(trx.to),
            transactionHash:  base_64_to_hex(trx.hash),
            blockNumber:block_number,
            timestamp: time_stamp,
        })
    } else {    
        None
    }
}

#[substreams::handlers::map]
fn map_trx(blk: eth::Block) -> Result<TransactionList, substreams::errors::Error> {
    let transaction_details_list = blk
        .transaction_traces
        .clone().into_iter()
        .filter(|trx| trx.status == TransactionTraceStatus::Succeeded as i32)
        .map(|trx| process_transaction_trace(trx, &blk))
        .collect();

    Ok(TransactionList {
        transaction_details_list,
    })
}

fn process_transaction_trace(trx: eth::TransactionTrace, block: &eth::Block) -> Transaction {
    let header = block.header.as_ref().unwrap();
    let block_number = block.number;
    let time_stamp = header.timestamp.clone().unwrap().seconds;
    Transaction {
        id:  base_64_to_hex(trx.hash),
        gasUsed: trx.gas_used,
        status: trx.status.to_string(),
        index: trx.index,
        nonce: trx.nonce,
        maxFeePerGas: option_bigint_to_number_u64(trx.max_fee_per_gas),
        maxPriorityFeePerGas: option_bigint_to_number_u64(trx.max_priority_fee_per_gas),
        gasLimit: trx.gas_limit,
        to: base_64_to_hex(trx.to),
        from: base_64_to_hex(trx.from),
        value: option_bigint_to_number_u64(trx.value),
        blockNumber: block_number,
        timestamp: time_stamp,
        gasPrice: option_bigint_to_number_u64(trx.gas_price)
    }
}

fn base_64_to_hex<T: std::convert::AsRef<[u8]>>(num:T) -> String {
    let num = hex::encode(&num);
    let num = num.to_string();
     format!("0x{}", &num)
}

fn option_bigint_to_number_string(bigint: Option<BigInt>) -> String {
    bigint
        .map(|num| {
            let bytes = num.bytes;
            let mut value: u128 = 0;

            for byte in bytes {
                value = (value << 8) + u128::from(byte);
            }

            value.to_string()
        })
        .unwrap_or_else(String::new)
}

fn option_bigint_to_number_u64(bigint: Option<BigInt>) -> u64 {
    bigint
        .map(|num| {
            let bytes = num.bytes;
            let mut value: u64 = 0;

            for byte in bytes {
                value = (value << 8) + u64::from(byte);
            }

            value
        })
        .unwrap_or(0)
}

#[substreams::handlers::map]
fn map_block(block: eth::Block) -> Result<BlockHeader, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();
    Ok(BlockHeader {
        id: base_64_to_hex(&block.hash),
        parentHash: base_64_to_hex(&header.parent_hash),
        uncleHash: base_64_to_hex(&header.parent_hash),
        receiptRoot: header.receipt_root.clone(),
        gasLimit: header.gas_limit,
        gasUsed: header.gas_used,
        number: block.number,
        nonce: header.nonce,
        difficulty: option_bigint_to_number_string(header.difficulty.clone()),
        totalDifficulty: option_bigint_to_number_string(header.total_difficulty.clone()) ,
        timestamp: header.timestamp.clone().unwrap().seconds,
        size: block.size,

    })
}

#[substreams::handlers::store]
fn store_price(transaction_details_list: TransactionList, store: StoreSetProto<Transaction>) {
    for transaction in transaction_details_list.transaction_details_list {
        store.set(transaction.blockNumber, format!("transaction.id"), &transaction);
    }
}

#[substreams::handlers::store]
fn store_block(block: BlockHeader, store: StoreSetProto<BlockHeader>) {
    store.set( block.number, format!("transaction.id"), &block);
}

#[substreams::handlers::map]
pub fn graph_out(map_trx: TransactionList, map_block: BlockHeader, map_contract: ContractList) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    db::register_transaction(&mut tables, &map_trx);
    db::create_block_entity(&mut tables, &map_block);
    db::register_contracts(&mut tables, &map_contract);
    Ok(tables.to_entity_changes())
}
 
