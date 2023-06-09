mod key;
mod pb;
mod db;
mod tables;

use pb::acme;
use crate::tables::Tables;
use pb::acme::{TranDetail, TransactionList, TransactionReceipt, Call, BlockHeader};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams::store::{StoreNew, StoreSetRaw};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::store::StoreSet;
use substreams::Hex;



substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_trx(blk: eth::Block) -> Result<TransactionList, substreams::errors::Error> {
    let transaction_details_list = blk
        .transaction_traces
        .into_iter()
        .filter(|trx| trx.status == TransactionTraceStatus::Succeeded as i32)
        .map(|trx| process_transaction_trace(trx))
        .collect();

    Ok(TransactionList {
        transaction_details_list,
    })
}

fn process_transaction_trace(trx: eth::TransactionTrace) -> TranDetail {
    let call_details_list = trx.calls.into_iter().map(|call| {
        Call {
            index: call.index,
            parent_index: call.parent_index,
            depth: call.depth,
            call_type: call.call_type,
            caller: call.caller,
            address: call.address,
            gas_limit: call.gas_limit,
            gas_consumed: call.gas_consumed,
            return_data: call.return_data,
            input: call.input,
            executed_code: call.executed_code,
            suicide: call.suicide,
            status_failed: call.status_failed,
            status_reverted: call.status_reverted,
            failure_reason: call.failure_reason,
            state_reverted: call.state_reverted,
            begin_ordinal: call.begin_ordinal,
            end_ordinal: call.end_ordinal,
        }
    }).collect();

    TranDetail {
        gas_used: trx.gas_used,
        status: trx.status.to_string(),
        block_hash: trx.hash,
        index: trx.index,
        begin_ordinal: trx.begin_ordinal,
        end_ordinal: trx.end_ordinal,
        nonce: trx.nonce,
        gas_limit: trx.gas_limit,
        to: trx.to,
        input: trx.input,
        r: trx.r,
        v: trx.v,
        s: trx.s,
        from: trx.from,
        public_key: trx.public_key,
        receipt: Some(TransactionReceipt {
            state_root: trx.receipt.as_ref().map(|r| r.state_root.clone()).unwrap_or_default(),
            cumulative_gas_used: trx.receipt.as_ref().map(|r| r.cumulative_gas_used).unwrap_or(0),
            logs_bloom: trx.receipt.as_ref().map(|r| r.logs_bloom.clone()).unwrap_or_default(),
        }),
        call_details_list,
    }
}

#[substreams::handlers::map]
fn map_block(block: eth::Block) -> Result<BlockHeader, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();

    Ok(BlockHeader {
        number: block.number,
        parent_hash: header.parent_hash.clone(),
        uncle_hash: header.uncle_hash.clone(),
        coinbase: header.coinbase.clone(),
        state_root: header.state_root.clone(),
        transactions_root: header.transactions_root.clone(),
        receipt_root: header.receipt_root.clone(),
        logs_bloom: header.logs_bloom.clone(),
        gas_limit: header.gas_limit,
        gas_used: header.gas_used,
        extra_data: header.extra_data.clone(),
        mix_hash: header.mix_hash.clone(),
        nonce: header.nonce,
        
    })
}

#[substreams::handlers::store]
fn store_price(transaction_details_list: TransactionList, output: StoreSetRaw) {
    for transaction in transaction_details_list.transaction_details_list {
        output.set(0, format!("transaction from:{}", &transaction.status), &transaction.status.to_string());
    }
}

#[substreams::handlers::map]
pub fn graph_out(map_trx: TransactionList) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    db::register_transaction(&mut tables, &map_trx);
    Ok(tables.to_entity_changes())
}
