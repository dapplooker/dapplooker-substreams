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
        .create_row("Transaction", format!("0x{}", &transaction.index))
        .set("status", format!("0x{}", &transaction.status))
        .set("gas", format!("0x{}", &transaction.gas_used))
        .set("status",  &transaction.block_hash)
        .set("index", format!("0x{}", &transaction.index))
        .set("begin_ordinal", format!("0x{}", &transaction.begin_ordinal))
        .set("end_ordinal", format!("0x{}", &transaction.end_ordinal));
}
