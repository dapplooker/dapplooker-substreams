use substreams_database_change::tables::Tables;
use hex;
use substreams::store::{
    self, DeltaProto, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto,
};
use substreams::Hex;
use prost_types::Value;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::BigInt;
use prost_types::Timestamp;


// create block entity
fn add_block_entity(tables: &mut Tables, blk: &eth::Block) {
    let header = blk.header.as_ref().unwrap();

    tables
        .create_row("blocks", blk.number.to_string())
        .set("hash", base_64_to_hex(blk.hash.clone()))
        .set(
            "parent_hash",
            Hex(&blk.header.as_ref().unwrap().parent_hash).to_string(),
        )
        .set(
            "timestamp",
            blk.header.as_ref().unwrap().timestamp.as_ref().unwrap(),
        )
        .set("size", blk.size)
        .set("nonce", header.nonce)
        .set("gas_limit", header.gas_limit)
        .set("gas_used", header.gas_used);
}

//create transaction entity
fn add_trx_info_entity(tables: &mut Tables, trx: &eth::TransactionTrace,  block_number: &u64,
    time_stamp: &Timestamp) {

    tables
    .create_row("transactions",  base_64_to_hex(trx.hash.clone()))
    .set("status", trx.status)
    .set("gas_used",  trx.gas_used)
    .set("gas_limit",  trx.gas_limit)
    .set("gas_price", option_bigint_to_number_u64(trx.gas_price.clone()))
    .set("nonce",  trx.nonce)
    .set("to_address",  base_64_to_hex(trx.to.clone()))
    .set("from_address",  base_64_to_hex(trx.from.clone()))
    .set("max_fee_per_gas",  option_bigint_to_number_u64(trx.max_fee_per_gas.clone()))
    .set("max_priority_fee_per_gas",  option_bigint_to_number_u64(trx.max_priority_fee_per_gas.clone()))
    .set("block_number",  block_number.clone())
    .set("timestamp",  time_stamp)
    .set("amount",  option_bigint_to_number_u64(trx.value.clone()));
}

//create contract entity
fn add_contracts_info_entity(tables: &mut Tables, trx: &eth::TransactionTrace,  block_number: &u64,
    time_stamp: &Timestamp) {

    tables
        .create_row("contracts",  base_64_to_hex(trx.to.clone()))
        .set("id", base_64_to_hex(trx.hash.clone()))
        .set("transaction_hash", base_64_to_hex(trx.hash.clone()))
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

 // main db_out function
#[substreams::handlers::map]
fn db_out(
    blk: eth::Block
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let header = &blk.header.as_ref().unwrap();
    let block_number = &blk.number;
    let time_stamp =  blk.header.as_ref().unwrap().timestamp.as_ref().unwrap();
    let mut tables = Tables::new();
    // get blocks data
    add_block_entity(&mut tables, &blk);
    for trx in &blk.transaction_traces{
        // get transactions data
        if(trx.status == TransactionTraceStatus::Succeeded as i32) {
            add_trx_info_entity(&mut tables, &trx, block_number, time_stamp);
        let contract_check = String::from_utf8_lossy(&trx.input).to_string();
         if (contract_check.starts_with("`�`@R") && base_64_to_hex(trx.to.clone()) != "0x0000000000000000000000000000000000000000" ) {
            //get contracts data
            add_contracts_info_entity(&mut tables, &trx, block_number, time_stamp);
        }
        }
    }
    Ok(tables.to_database_changes())
}
