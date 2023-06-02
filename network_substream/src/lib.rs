mod pb; 

use pb::acme;
use crate::acme::{TranDetail, TranasactionList};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;   
use substreams::errors::Error;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_trx(blk: eth::Block) -> Result<TranasactionList, substreams::errors::Error> {
    let mut transaction_details_list: Vec<TranDetail> = vec![];
    for trx in blk.transaction_traces {
        if trx.status == TransactionTraceStatus::Succeeded as i32 {
            transaction_details_list.push(TranDetail {
                // Get the from address.
           from_address : String::from_utf8(trx.from).unwrap(),
           // Get the to address.
           to_address : String::from_utf8(trx.to).unwrap(),
           // Get the gas used.
           gas_used : trx.gas_used,
           status : trx.status.to_string(),
           // // Get the block hash.
           block_hash : trx.hash,
           // max_fee_per_gas : trx.max_fee_per_gas,
           // max_priority_fee_per_gas : trx.max_priority_fee_per_gas,
           index : trx.index,
           begin_ordinal : trx.begin_ordinal,
           end_ordinal : trx.end_ordinal,
            })
        }
    }
    Ok(TranasactionList {
        transaction_details_list,
    })
}


// #[substreams::handlers::map]
// fn map_trx(blk: eth::Block) -> Result<TranDetail, substreams::errors::Error> {
//     let mut transaction_details:TranDetails;
//     for trx in blk.transaction_traces {
//         if trx.status == TransactionTraceStatus::Succeeded as i32 {
//             let tran_detail = TranDetail{
//             // Get the from address.
//            from_address : String::from_utf8(trx.from).unwrap(),
//             // Get the to address.
//             to_address : String::from_utf8(trx.to).unwrap(),
//             // Get the gas used.
//             gas_used : trx.gas_used,
//             status : trx.status.to_string(),
//             // // Get the block hash.
//             block_hash : trx.hash,
//             // max_fee_per_gas : trx.max_fee_per_gas,
//             // max_priority_fee_per_gas : trx.max_priority_fee_per_gas,
//             index : trx.index,
//             begin_ordinal : trx.begin_ordinal,
//             end_ordinal : trx.end_ordinal,
//             };
//             return Ok(tran_detail);
//         }
//     }
//     // Return an error if no transaction traces were found.
//     let x :&str= "No transaction traces found";
//     Err(substreams::errors::Error::Unexpected(x.to_string()))
// }





