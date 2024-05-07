// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChanges {
    #[prost(message, repeated, tag="1")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_balance: ::prost::alloc::string::String,
    #[prost(enumeration="BalanceChangeType", tag="9")]
    pub change_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceChangeType {
    /// cannot determine balance change
    TypeUnknown = 0,
    /// easy case where storage change is in the same call as the Transfer call
    Type1 = 1,
    /// storage change is in a different call than the Transfer call
    Type2 = 2,
}
impl BalanceChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceChangeType::TypeUnknown => "TYPE_UNKNOWN",
            BalanceChangeType::Type1 => "TYPE_1",
            BalanceChangeType::Type2 => "TYPE_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNKNOWN" => Some(Self::TypeUnknown),
            "TYPE_1" => Some(Self::Type1),
            "TYPE_2" => Some(Self::Type2),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
