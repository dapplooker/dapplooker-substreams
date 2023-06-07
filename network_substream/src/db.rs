use substreams::scalar::BigInt;
use substreams::store::{ DeltaBigInt, Deltas};

use crate::key;
use crate::acme::{TranDetail, TransactionList};
use crate::tables::Tables;

pub fn register_transaction(tables: &mut Tables, transactions: &TransactionList) {
    for transaction in &transactions.transaction_details_list {
        create_transaction_entity(tables, transaction);
    }
}

fn create_transaction_entity(tables: &mut Tables, transaction: &TranDetail) {
    tables
        .create_row("TranDetail", format!("0x{}", &transaction.index))
        .set("status", &transaction.status)
        .set("gas_used",  transaction.gas_used)
        .set("block_hash",  &transaction.block_hash)
        .set("index",transaction.index)
        .set("begin_ordinal", transaction.begin_ordinal)
        .set("end_ordinal",  transaction.end_ordinal);
}
