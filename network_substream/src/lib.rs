mod key;
mod pb;
mod db;
mod tables;

use pb::acme;
use crate::tables::Tables;
use pb::acme::{TranDetail, TransactionList};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams::store::StoreNew;
use substreams::store::StoreSetRaw;
use substreams::store::StoreSet;
use substreams_entity_change::pb::entity::EntityChanges;


substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_trx(blk: eth::Block) -> Result<TransactionList, substreams::errors::Error> {
    let mut transaction_details_list: Vec<TranDetail> = vec![];
    for trx in blk.transaction_traces {
        if trx.status == TransactionTraceStatus::Succeeded as i32 {
            transaction_details_list.push(TranDetail {
                gas_used : trx.gas_used,
                status : trx.status.to_string(),
                block_hash : trx.hash,
                index : trx.index,
                begin_ordinal : trx.begin_ordinal,
                end_ordinal : trx.end_ordinal
            })
        }
    }
    Ok(TransactionList {
        transaction_details_list
    })
}


#[substreams::handlers::store]
fn store_price(
    transaction_details_list: TransactionList,
    output: StoreSetRaw) {
    for transaction in transaction_details_list.transaction_details_list {
        output.set(0, format!("transaction from:{}", &transaction.status), &transaction.status.to_string())
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    map_trx: TransactionList, /* map_pools_registered */
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    db::register_transaction(&mut tables, &map_trx);
    Ok(tables.to_entity_changes())
}


