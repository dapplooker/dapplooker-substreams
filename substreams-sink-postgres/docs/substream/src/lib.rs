mod pb;

use substreams_database_change::tables::Tables;
use hex;    
use pb::acme::{BlockMeta, Transaction, TransactionList, Contract, ContractList};
use substreams::store::{
    self, DeltaProto, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto,
};
use substreams::Hex;
use prost_types::Value;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::pb as ethpb;
use substreams_ethereum::pb::eth::v2::BigInt;

// create block entity
fn add_block_entity(tables: &mut Tables, blk: &eth::Block) {
    let block_hash = Hex(&blk.hash).to_string();
    let header = blk.header.as_ref().unwrap();

    tables
        .create_row("block_meta", base_64_to_hex(block_hash.clone()))
        .set("id", blk.hash.clone())
        .set("number", blk.number)
        .set("parent_hash", Hex(&blk.header.as_ref().unwrap().parent_hash).to_string(),)
        .set("timestamp", blk.header.as_ref().unwrap().timestamp.as_ref().unwrap(),)
        .set("size", blk.size)
        .set("nonce", header.nonce)
        .set("receipt_root", header.receipt_root.clone())
        .set("gas_limit", header.gas_limit)
        .set("gas_used", header.gas_used);
}

//create transaction entity
fn add_trx_info_entity(tables: &mut Tables, trx: &eth::TransactionTrace,  block_number: &u64,
    time_stamp: i64,) {
    
    tables
    .create_row("transactions",  base_64_to_hex(trx.hash.clone()))
    .set("id", trx.hash.clone())
    .set("status", trx.status)
    .set("gas_used",  trx.gas_used)
    .set("gas_limit",  trx.gas_limit)
    .set("transaction_nonce",  trx.nonce)
    .set("to_address",  base_64_to_hex(trx.to.clone()))
    .set("from_address",  base_64_to_hex(trx.from.clone()))
    .set("max_fee_per_gas",  option_bigint_to_number_string(trx.max_fee_per_gas.clone()))
    .set("max_priority_fee_per_gas",  option_bigint_to_number_string(trx.max_priority_fee_per_gas.clone()))
    .set("block_number",  block_number.clone())
    .set("timestamp",  time_stamp);
}

//create contract entity
fn add_contracts_info_entity(tables: &mut Tables, trx: &eth::TransactionTrace,  block_number: &u64,
    time_stamp: i64,) {
    
    tables
        .create_row("contracts",  base_64_to_hex(trx.hash.clone()))
        .set("id", trx.hash.clone())
        .set("transaction_hash", trx.hash.clone())
        .set("address",  base_64_to_hex(trx.to.clone()))
        .set("owner",  base_64_to_hex(trx.from.clone()))
        .set("block_number",  block_number.clone())
        .set("timestamp",  time_stamp);
}

//base64 to hex
fn base_64_to_hex<T: std::convert::AsRef<[u8]>>(num:T) -> String {
    let num = hex::encode(&num);
    let num = num.to_string();
     format!("0x{}", &num)
}

// bigint to string
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

 // main db_out function
#[substreams::handlers::map]
fn db_out(
    blk: eth::Block
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let header = &blk.header.as_ref().unwrap();
    let block_number = &blk.number;
    let time_stamp = header.timestamp.clone().unwrap().seconds;
    let mut tables = Tables::new();
    // get blocks data
    add_block_entity(&mut tables, &blk);
    for trx in &blk.transaction_traces{
        // get transactions data
        log::info!("hello");
        if(trx.status == TransactionTraceStatus::Succeeded as i32) {
            add_trx_info_entity(&mut tables, &trx, block_number, time_stamp);
        let contract_check = String::from_utf8_lossy(&trx.input).to_string();
        if contract_check.starts_with("`�`@R"){
            //get contracts data
            add_contracts_info_entity(&mut tables, &trx, block_number, time_stamp);
        }
        }
    }
    Ok(tables.to_database_changes())
}

