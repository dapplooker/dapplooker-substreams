use substreams::scalar::BigInt;
use substreams::store::{ DeltaBigInt, Deltas};

use crate::key;
use crate::acme::{TranDetail, TranasactionList};
use crate::tables::Tables;

pub fn register_transaction(tables: &mut Tables, transactions: &TranasactionList) {
    for transaction in &transactions.transaction_details_list {
        create_transaction_entity(tables, transaction);
    }
}

fn create_transaction_entity(tables: &mut Tables, transaction: &TranDetail) {
    tables
        .create_row("transaction", format!("0x{}", &transaction.index))
        .set("address", format!("0x{}", &transaction.from_address));
}