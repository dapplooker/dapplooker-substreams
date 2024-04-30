use crate::abi;
use crate::pb::erc20::types::v1::Erc20Token;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::rpc::RpcBatch;

pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}

pub fn get_erc20_token(token_address: &String) -> Option<Erc20Token> {
    let batch = RpcBatch::new();
    let responses = batch
        .add(
            abi::erc20::functions::Decimals {},
            Hex::decode(token_address).unwrap(),
        )
        .add(
            abi::erc20::functions::Name {},
            Hex::decode(token_address).unwrap(),
        )
        .add(
            abi::erc20::functions::Symbol {},
            Hex::decode(token_address).unwrap(),
        )
        .execute()
        .unwrap()
        .responses;

    let decimals: u64;
    match RpcBatch::decode::<_, abi::erc20::functions::Decimals>(&responses[0]) {
        Some(decoded_decimals) => {
            let max_str: String = u64::MAX.to_string();
            let big_int_limit = BigInt::try_from(max_str).unwrap();
            let max_check = decoded_decimals.min(big_int_limit);

            decimals = max_check.to_u64();
        }
        None => {
            decimals = BigInt::zero().to_u64();
        }
    };

    let name: String;
    match RpcBatch::decode::<_, abi::erc20::functions::Name>(&responses[1]) {
        Some(decoded_name) => {
            name = decoded_name;
        }
        None => {
            name = String::new();
        }
    };

    let symbol: String;
    match RpcBatch::decode::<_, abi::erc20::functions::Symbol>(&responses[2]) {
        Some(decoded_symbol) => {
            symbol = decoded_symbol;
        }
        None => {
            symbol = String::new();
        }
    };

    return Some(Erc20Token {
        address: token_address.clone(),
        name,
        symbol,
        decimals,
    });
}
